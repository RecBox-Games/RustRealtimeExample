use std::collections::HashMap;

use ggez::{graphics, Context};

use crate::standard_deck::StandardDeckResources;

//////// GameResources ////////
pub struct GameResources {
    pub deck_res: StandardDeckResources,
    card_placeholder: graphics::Image,
    // text_graphics: we will store rendered text into hashmap so that we don't
    // render text every frame which is expensive
    // (I'm actually not sure if this is helpful in ggez 0.9.3 but it was in 0.7.0)
    text_graphics: HashMap<String, graphics::Text>,
}


impl GameResources {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            deck_res: StandardDeckResources::new(ctx),
            card_placeholder: graphics::Image::from_path(ctx, "/card_none.png").unwrap(),
            text_graphics: HashMap::new(),
        }
    }
        
    pub fn get_placeholder(&self) -> &graphics::Image {
        &self.card_placeholder
    }

    pub fn get_text_graphic(&mut self, text: &str) -> &graphics::Text {
        if self.text_graphics.get(text).is_none() {
            let fragment = graphics::TextFragment::new(text)
                .color((0.0, 0.0, 0.0))
                .scale(32.0);
            let new_graphic = graphics::Text::new(fragment);
            self.text_graphics.insert(text.to_string(), new_graphic);
        } 
        return self.text_graphics.get(text).unwrap();
    }

}
