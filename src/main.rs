use ggez::{
    event,
    graphics,
    Context, GameResult, input::keyboard::KeyInput,
};
use std::path;

mod progress;
mod my_card_game;
use my_card_game::*;
mod standard_deck;
use standard_deck::*;
mod resources;
use resources::*;

// screen width
#[cfg(debug_assertions)]
const SCREEN_WIDTH: f32 = 1200.0;
#[cfg(not(debug_assertions))]
const SCREEN_WIDTH: f32 = 1920.0;

// screen height
#[cfg(debug_assertions)]
const SCREEN_HEIGHT: f32 = 800.0;
#[cfg(not(debug_assertions))]
const SCREEN_HEIGHT: f32 = 1080.0;


struct MainState {
    resources: GameResources,
    card_game: MyCardGame,
    client_handles: Vec<String>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState {
            resources: GameResources::new(ctx),
            card_game: MyCardGame::new(),
            client_handles: vec![],
        };
        Ok(state)
    }

    fn track_controlpad_clients(&mut self) {
        if let Ok(true) = controlpads::clients_changed() {
            if let Ok(handles) = controlpads::get_client_handles() {
                self.client_handles = handles;
            } else {
                println!("Warning: Failed to get client handles");
            }
        }
    }

    fn receive_controlpad_messages(&mut self) -> Vec<(String, String)> {
        self.track_controlpad_clients();
        let mut messages: Vec<(String, String)> = Vec::new();
        for handle in &self.client_handles {
            if let Ok(msgs) = controlpads::get_messages(handle) {
                for msg in &msgs {
                    messages.push((handle.to_string(), msg.to_string()));
                }
            } else {
                println!("WARNING: Error while gatting controlad messages");
            }
        }
        messages
    }
}


// By implementing EventHandler, we are making MainState play nicely with
// ggez's game loop. When we call event::run() in main with a MainState passed
// in, we are starting a loop where update() and draw() are called repeatedly
impl event::EventHandler<ggez::GameError> for MainState {

    // called once per frame (synchronous with MainState::draw())
    // default 60 frames per second
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // handle received controlpad messages
        for (client, msg) in self.receive_controlpad_messages() {
            self.card_game.handle_controlpad_message(client, msg);
        }
        // update game
        self.card_game.update();
        Ok(())
    }

    // called once per frame (synchronous with MainState::update())
    // default 60 frames per second
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.4, 0.9, 0.6, 1.0]));
        // make things pixely instead of blury
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        // draw MyCardGame
        self.card_game.draw(&mut canvas, ctx, &mut self.resources)?;
        // finished drawing, show it all on the screen!
        canvas.finish(ctx)?;
        Ok(())
    }


    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(keycode) = input.keycode {
            self.card_game.handle_key_press(keycode);
        }
        Ok(())
    }    
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    let cb = ggez::ContextBuilder::new("drawing", "ggez")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default()
                     .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
                     .resizable(true)
        );
    let (mut ctx, events_loop) = cb.build()?;
    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
