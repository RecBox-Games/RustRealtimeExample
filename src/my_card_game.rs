use crate::standard_deck::*;

mod draw_my_card_game;

use crate::progress::*;
use ggez::input::keyboard::KeyCode;
use rand::seq::SliceRandom;
use rand::thread_rng;


//////// Helpers ////////
fn parse_card(card_str: &str) -> Option<(&str, &str, &str)> {
    let mut parts = card_str.split(",");
    let side = parts.next()?;
    let suit = parts.next()?;
    let rank = parts.next()?;
    return Some((side, suit, rank));
}

//////// Deck ////////
struct Deck {
    cards: Vec<CardSpec>,
}

impl Deck {
    // randomized 52 cards
    fn new() -> Self {
        let mut cards: Vec<CardSpec> = Vec::new();
        for suit in CARD_SUITS.iter().map(|x| x.to_str()) {
            for rank in CARD_RANKS.iter().map(|x| x.to_str()) {
                let card_spec = CardSpec::from_strs(suit, rank);
                cards.push(card_spec);
            }
        }
        let mut deck = Self {
            cards
        };
        deck.shuffle();
        deck
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}


//////// SplayProgression ////////
const SPLAY_RISE_TIME: f32 = 0.2;
const SPLAY_FLIP_TIME: f32 = 0.4;
const SPLAY_TRAVEL_TIME: f32 = 1.5;
enum SplayProgression {
    Rise(Progression),
    Flip(Progression),
    Travel(Progression),
    //Lower(Progression),
}

impl SplayProgression {
    fn new() -> Self {
        use SplayProgression::*;
        Rise(Progression::new(SPLAY_RISE_TIME))
    }

    fn update(&mut self) {
        use SplayProgression::*;
        match self {
            Rise(p) => {
                p.update();
                if p.is_done() {
                   *self = Flip(Progression::new(SPLAY_FLIP_TIME)); 
                }
            }
            Flip(p) => {
                p.update();
                if p.is_done() {
                   *self = Travel(Progression::new(SPLAY_TRAVEL_TIME)); 
                }
            }
            Travel(p) => {
                p.update();
            }
        }
    }

    fn is_done(&self) -> bool {
        match self {
            SplayProgression::Travel(p) => p.is_done(),
            _ => false,
        }
    }
}


//////// Player ////////
struct Player {
    handle: String,
    name: String,
    left_card: Option<CardSpec>,
    right_card: Option<CardSpec>,
}

impl Player {
    fn state_string(&self) -> String {
        let lcard_state = self.left_card.map_or("".to_string(), |x| x.to_string());
        let rcard_state = self.right_card.map_or("".to_string(), |x| x.to_string());
        format!("{}:{}:{}", &self.name, lcard_state, rcard_state)
    }

    fn send_message(&self, msg: &str) {
        controlpads::send_message(&self.handle, msg)
            .unwrap_or_else(|e| println!("WARNING: Error sending controlpad message: {}", e));

    }
    
    fn send_state(&self) {
        self.send_message(&format!("state:playing:{}", self.state_string()));
    }

    fn revoke_card(&mut self, is_left: bool) {
        if is_left {
            self.left_card = None;
        } else {
            self.right_card = None;

        };
    }
}


// Notice that the members of MyCardGame have no data for graphics. This is
// so that the state of the game is separate from the representation (graphics)
// of the game. This is a personal design choice and you can do things
// differently if you please.
// - The "state" of the game is handled in this file and the "representation"
//   of the game is handled in draw_my_card_game.rs
pub struct MyCardGame {
    //// cards
    // deck: cards in the facedown deck in the center of the screen
    deck: Deck,
    // splayed_cards: cards at the top of the screen
    splayed_cards: Vec<CardSpec>,
    // splaying_cards: the card traveling from the deck to the splayed_cards area
    splaying_cards: Vec<(CardSpec, SplayProgression)>,
    // center_card: card in the center of the screen next to the deck
    center_card: CardSpec,
    // giving_card: facedown card that goes off the bottom of the screen to go to the player
    giving_card: Option<(String, Progression)>,
    ////
    //players:
    players: Vec<Player>,
}

const GIVING_TRAVEL_TIME: f32 = 1.0;

impl MyCardGame {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        let center_card = deck.cards.pop().unwrap();
        Self {
            deck,
            splayed_cards: Vec::new(),
            splaying_cards: Vec::new(),
            center_card,
            giving_card: None,
            players: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // update splaying card
        let mut i = 0;
        while i < self.splaying_cards.len() {
            let (card_spec, splay_p) = &mut self.splaying_cards[i];
            if splay_p.is_done() {
                self.splayed_cards.push(*card_spec);
                self.splaying_cards.remove(i);
            } else {
                splay_p.update();
                i += 1;
            };
        }
        // update giving card
        let mut give_finish: Option<String> = None; // get around borrowing rules
        if let Some((player_handle, prog)) = &mut self.giving_card {
            prog.update();
            if prog.is_done() {
                give_finish = Some(player_handle.clone());
                self.giving_card = None;
            }
        }
        if let Some(player_handle) = give_finish {
            self.finish_give_card(&player_handle);
        }
    }

    fn deal(&mut self) {
        if let Some(next_card) = self.deck.cards.pop() {
            self.splaying_cards.push((next_card, SplayProgression::new()));
        }
    }

    // Assumes player_handle is a valid handle for a player in self.players
    fn start_give_card(&mut self, player_handle: String, side: &str, suit: &str, rank: &str) {
        if self.giving_card.is_some() {
            return;
        }
        let player = self.players.iter_mut().find(|x| x.handle == player_handle).unwrap();
        if let Some(next_card) = self.deck.cards.pop() {
            self.center_card = CardSpec::from_strs(suit, rank);
            player.revoke_card(side == "L");
            // we set the card here, but we won't tell the player about it (via
            // send_state()) until the giving_card progresses across the screen
            if side == "L" {
                player.left_card = Some(next_card);
            } else {
                player.right_card = Some(next_card);
            }
            self.giving_card = Some((player.handle.clone(), Progression::new(GIVING_TRAVEL_TIME)));
        }
    }

    fn finish_give_card(&mut self, player_handle: &str) {
        if let Some(player) = self.players.iter().find(|x| x.handle == player_handle) {
            player.send_state();
        }
    }
    
    
    pub fn handle_key_press(&mut self, _key: KeyCode) {
        self.deal();

        //self.start_give_card();
    }

    pub fn handle_controlpad_message(&mut self, client: String, message: String) {
        let mut parts = message.split(":");
        let msg_type = parts.next().unwrap(); // first on a split is always some
        if let Some(player) = self.players.iter_mut().find(|x| x.handle == client) {
            match msg_type {
                "state-request" => {
                    // a state request after the player is already joined
                    player.send_state();
                }
                "deal" => {
                    self.deal();
                }
                "card" => {
                    let card_parse = parse_card(parts.next().unwrap_or(""));
                    let (side, suit, rank) = if let Some((a, b, c)) = card_parse { (a,b,c) } else {return};
                    let handle = player.handle.clone();
                    self.start_give_card(handle, side, suit, rank);
                }
                _ => println!("WARNING: bad player message: {}", &message),
            }
        } else if msg_type == "state-request" {
            // a state request before the player has joined
            controlpads::send_message(&client, "state:joining")
                .unwrap_or_else(|e| println!("WARNING: failed to send message: {}", e));
        } else if msg_type == "join" {
            let new_player = Player {
                handle: client,
                name: parts.next().unwrap_or("").to_string(),
                left_card: self.deck.cards.pop(),
                right_card: self.deck.cards.pop(),
            };
            new_player.send_state();
            self.players.push(new_player);
        } else {
            println!("WARNING: a controlpad tried to send something other than \
                      'join' when it hadn't joined yet");
        }
        
    }
    
}



