extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    println!("Testing PistonWindow initialization...");
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        // [800, 600]
        [to_coord_u32(width), to_coord_u32(height)]
    ).exit_on_esc(true)
        .build()
        .expect("Failed to initialize PistonWindow");
    println!("Window initialized successfully!");

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|args| {
            game.update(args.dt);
        });
    }
}
