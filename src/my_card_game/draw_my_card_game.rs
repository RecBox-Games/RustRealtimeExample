use super::*;
use ggez::{
    graphics::{Canvas, DrawParam, Image}, Context, GameResult,
};
use glam::Vec2;
use std::f32::consts::PI;

use crate::resources::*;

// TODO: move to sahred location
const CARD_IMG_WIDTH: f32 = 148.0;
const CARD_IMG_HEIGHT: f32 = 148.0;

// screen locations
const SPLAYED_CARD_DISTANCE: f32 = 30.0;



//////// Deck ////////
impl Deck {
    fn deck_height(&self) -> usize {
        return ((self.cards.len() + 5) / 6) as usize
    }

    fn facedown_card_offset(&self, n: usize) -> Vec2 {
        Vec2::new(0.0, n as f32 * -2.0)
    }
    
    pub fn draw(&self, canvas: &mut Canvas, location: Vec2, res: &GameResources) {
        // draw card_none.png to represent an empty deck
        if self.cards.len() == 0 {
            canvas.draw(res.get_placeholder(), location);
            return;
        }
        // draw cards up to a certain height depending how many cards are left
        for i in 0..self.deck_height() {
            canvas.draw(res.deck_res.get_back_image(), location + self.facedown_card_offset(i));
        }
    }

    fn top_offset(&self) -> Vec2 {
        self.facedown_card_offset(self.deck_height())
    }
}

//////// MyCardGame ////////
impl MyCardGame {
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, res: &mut GameResources) -> GameResult<()> {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        // draw splayed cards
        let splayed_cards_loc = Vec2::new( 0.0, 40.0 );
        for (i, card_spec) in self.splayed_cards.iter().enumerate() {
            let card_loc = splayed_cards_loc + Vec2::new(SPLAYED_CARD_DISTANCE * i as f32, 0.0);
            canvas.draw(res.deck_res.get_card_image(card_spec), card_loc);
        }
        //
        // draw center card
        let center_card_loc = Vec2::new(
            (screen_width - CARD_IMG_WIDTH) / 2.0,
            (screen_height - CARD_IMG_HEIGHT) / 2.0,
        );
        canvas.draw(res.deck_res.get_card_image(&self.center_card), center_card_loc);
        //
        // draw deck
        let deck_loc = center_card_loc + Vec2::new(CARD_IMG_WIDTH, 0.0);
        self.deck.draw(canvas, deck_loc, res);
        //
        // draw splaying cards
        for i in (0..self.splaying_cards.len()).rev() {
            let card_start = deck_loc + self.deck.top_offset();
            let card_end = splayed_cards_loc + Vec2::new(SPLAYED_CARD_DISTANCE*(self.splayed_cards.len() + i) as f32, 0.0);
            let (card_spec, splay_p) = &self.splaying_cards[i];
            let card_img = res.deck_res.get_card_image(&card_spec);
            let back_img = res.deck_res.get_back_image();
            draw_splaying_card(canvas, splay_p, card_start, card_end, card_img, back_img);
        }
        //
        // draw giving card
        if let Some((_, prog)) = &self.giving_card {
            let start_loc = deck_loc + self.deck.top_offset();
            let end_loc = Vec2::new(screen_width*0.6, screen_height);
            let giving_loc = giving_card_loc(start_loc, end_loc, prog.progress());
            canvas.draw(res.deck_res.get_back_image(), giving_loc);
        }
        //
        // draw player names
        let mut name_loc = Vec2::new(20.0, 200.0);
        for player in &self.players {
            canvas.draw(res.get_text_graphic(&player.name), name_loc);
            name_loc += Vec2::new(0.0, 40.0);
        }
        Ok(())
    }

    
}

fn draw_splaying_card(canvas: &mut Canvas, splay_progress: &SplayProgression,
                          start_loc: Vec2, end_loc: Vec2,
                          card_img: &Image, back_img: &Image) {
    use SplayProgression::*;
    match splay_progress {
        Rise(p) => {
            let rising_loc = start_loc - p.progress() * Vec2::new(0.0, 12.0);
            canvas.draw(back_img, rising_loc);
        }
        Flip(p) => {
            let risen_loc = start_loc - Vec2::new(0.0, 12.0);
            let flip_scale = (p.progress() * PI).cos().abs();
            let flip_offset = (1.0 - flip_scale)*0.5*CARD_IMG_WIDTH;
            let flip_loc = risen_loc + Vec2::new(flip_offset, 0.0);
            let card_side = if p.progress() < 0.5 {
                back_img
            } else {
                card_img
            };
            canvas.draw(card_side, DrawParam::default()
                        .dest(flip_loc)
                        .scale(Vec2::new(flip_scale, 1.0)));
        }
        Travel(p) => {
            let risen_loc = start_loc - Vec2::new(0.0, 12.0);
            let travel_loc = interpolate(risen_loc, end_loc, Interpolation::Natural, p.progress());
            canvas.draw(card_img, travel_loc);
        }
    }
}



fn giving_card_loc(start_loc: Vec2, end_loc: Vec2, p: f32) -> Vec2 {
    let lift_loc = start_loc + Vec2::new(3.0, -15.0);
    // this interpolation will quickly progress toward lift_loc while slowly
    // progressing towards end_loc then, as it progrsses, start to move more
    // quickly towards end_loc and more slowly towards lift_loc
    interpolate2(start_loc, lift_loc, end_loc,
                 Interpolation::SlowDown, Interpolation::SpeedUp,
                 p)
}

enum Interpolation {
    Linear,
    SlowDown,
    SpeedUp,
    RoundStart,
    RoundEnd,
    RoundFull,
    Natural,
}

// interpolation between two points (use Interpolation::Linear for constant speed)
fn interpolate(start_loc: Vec2, end_loc: Vec2, interp: Interpolation, progress: f32) -> Vec2 {
    use Interpolation::*;
    let p = match interp {
        Linear => progress,
        SlowDown => progress.sqrt(),
        SpeedUp => progress.powf(2.0),
        RoundStart => 1.0 - (progress * PI/2.0).cos(),
        RoundEnd => (progress * PI/2.0).sin(),
        RoundFull =>  (1.0 + ((1.0 - progress) * PI).cos())/2.0,
        Natural => {
            let roundfull = (1.0 + ((1.0 - progress) * PI).cos())/2.0;
            let roundend = (progress * PI/2.0).sin();
            (roundfull + roundend)/2.0
        }
    };
    //
    (1.0 - p) * start_loc  +  p * end_loc
}


// interpolate along a curved path between start_loc and end_loc that curves
// towards intermediary (the final interpolation is linear, change the function
// if you want to do something different)
fn interpolate2(start_loc: Vec2, intermediary_loc: Vec2, end_loc: Vec2,
                interp1: Interpolation, interp2: Interpolation, progress: f32) -> Vec2 {
    let interp_a = interpolate(start_loc, intermediary_loc, interp1, progress);
    let interp_b = interpolate(intermediary_loc, end_loc, interp2, progress);
    return interpolate(interp_a, interp_b, Interpolation::Linear, progress);
}
