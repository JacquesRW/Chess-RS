mod model;
mod controller;
mod draw;
mod icons;
use crate::model::helper::*;
use crate::model::pieces::*;
use crate::model::defs::*;

#[cfg(test)]
mod puzzles; // de facto testing module now

pub const _START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const _TEST_POSITIONS: [&str;1] = ["r2qk2r/ppp4p/3p1pn1/3Pn1p1/2B1P3/2N2P1P/PP2QP2/R4RK1 w kq - 1 15"];

use log::error;
use model::defs::Board;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

// gravity strength
pub const G: f64 = 10.0;
// simulation speed multiplier
pub const TSCALE: f64 = 2.0;
// displayed area dimensions
pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 64;
// scale of window relative to above dimensions
pub const SCALE: f64 = 10.0;

fn main()  -> Result<(), Error> { 
    let player_color = BLACK;
    let mut game = Board::from_fen(_START_POSITION);  
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64  * SCALE);
        WindowBuilder::new()
            .with_title("physics-rs")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let check: Option<bool>;
        if let Event::RedrawRequested(_) = event {
            game.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P)  { 
                if game.color == player_color {
                    let moves = game.find_all_possible_moves();
                    loop {
                        let mut a = String::new();
                        std::io::stdin().read_line(&mut a).unwrap();
                        // collecting input into a vector split at whitespace
                        let coords: Vec<&str> = a.split_whitespace().collect();
                        // getting origin of move
                        let origin = get_square(&coords[0]);
                        // getting destination of move
                        let destination = get_square(&coords[1]);
                        // getting target of move
                        let m = Move { target: game.board[origin[0]][origin[1]], orig: origin, dest: destination};
                        // validating input
                        if moves.iter().any(|&i| i==m) {
                            check = game.make_move(m);
                            break;
                        }
                        else {
                            println!("Please enter a valid move:")
                        }
                    }
                }
                else {check = game.engine_move()}
                if check.is_some() {
                    // checkmate
                    if check.unwrap() {
                        println!("Checkmate, {} has won!", match game.color { WHITE => "black", BLACK => "white", _ => panic!("Invalid colour!")})
                    }
                    // stalemate
                    if !check.unwrap() {
                        println!("Stalemate!")
                    }
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                window.request_redraw()
            }
        }
    });
}