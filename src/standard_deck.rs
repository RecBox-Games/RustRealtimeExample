use std::collections::HashMap;

use CardSuit::*;
use CardRank::*;
use ggez::{Context, graphics};

pub const CARD_SUITS: [CardSuit; 4] = [ Heart, Diamond, Spade, Club ];
pub const CARD_RANKS: [CardRank; 13] = [ _02, _03, _04, _05, _06, _07, _08, _09,
                                          _10, _J, _Q, _K, _A, ];

//////// CardSuit ////////
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum CardSuit {
    Heart,
    Diamond,
    Spade,
    Club
}

impl CardSuit {
    pub fn from_str(s: &str) -> Self {
        match s {
            "hearts" => Heart,
            "diamonds" => Diamond,
            "spades" => Spade,
            "clubs" => Club,
            _ => panic!("bad suit: {}", s)
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Heart => "hearts",
            Diamond => "diamonds",
            Spade => "spades",
            Club => "clubs",
        }
    }
}

//////// CardRank ////////
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum CardRank {
    _02,
    _03,
    _04,
    _05,
    _06,
    _07,
    _08,
    _09,
    _10,
    _J,
    _Q,
    _K,
    _A,
}

impl CardRank {
    pub fn from_str(s: &str) -> Self {
        match s {
            "02" => _02,
            "03" => _03,
            "04" => _04,
            "05" => _05,
            "06" => _06,
            "07" => _07,
            "08" => _08,
            "09" => _09,
            "10" => _10,
            "J" => _J,
            "Q" => _Q,
            "K" => _K,
            "A" => _A,
            _ => panic!("bad rank: {}", s)
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            _02 => "02",
            _03 => "03",
            _04 => "04",
            _05 => "05",
            _06 => "06",
            _07 => "07",
            _08 => "08",
            _09 => "09",
            _10 => "10",
            _J => "J",
            _Q => "Q",
            _K => "K",
            _A => "A",
        }
    }
}

//////// CardSpec ////////
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct CardSpec {
    suit: CardSuit,
    rank: CardRank,
}
impl CardSpec {
    pub fn from_strs(suit: &str, rank: &str) -> Self {
        Self {
            suit: CardSuit::from_str(suit),
            rank: CardRank::from_str(rank),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{}", self.suit.to_str(), self.rank.to_str())
    }
}


//////// StandardDeckResources ////////
pub struct StandardDeckResources {
    card_fronts: HashMap<CardSpec, graphics::Image>,
    card_back: graphics::Image,
}

impl StandardDeckResources {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            card_fronts: load_card_fronts(ctx),
            card_back: graphics::Image::from_path(ctx, "/card_back.png").unwrap(),
        }
    }

    pub fn get_card_image(&self, card: &CardSpec) -> &graphics::Image {
        self.card_fronts.get(card).unwrap()
    }

    pub fn get_back_image(&self) -> &graphics::Image {
        &self.card_back
    }
}

fn load_card_fronts(ctx: &mut Context) -> HashMap<CardSpec, graphics::Image> {
    let mut img_map: HashMap<CardSpec, graphics::Image> = HashMap::new();
    // loop through every suit+rank combo and load that image from resources/card_fronts/
    for suit in CARD_SUITS.iter().map(|x| x.to_str()) {
        for rank in CARD_RANKS.iter().map(|x| x.to_str()) {
            let card_spec = CardSpec::from_strs(suit, rank);
            let img_path = format!("/card_fronts/card_{}_{}.png", suit, rank);
            // panic if we fail to load any image
            let img = graphics::Image::from_path(ctx, &img_path).unwrap();
            img_map.insert(card_spec, img);
        }
    }
    img_map
}
