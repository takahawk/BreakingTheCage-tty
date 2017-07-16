/*
    MIT License

    Copyright (c) 2017 Dan Hawk

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

extern crate breaking_the_cage as core;
extern crate termion;

mod render;
mod control;

use std::io;

use core::World;

#[derive(PartialEq)]
pub enum GameStatus {
    RUNNING,
    EXIT,
}

pub struct GameState {
    status: GameStatus,
}

pub fn main_loop() -> io::Result<()> {
    let mut state = GameState { status: GameStatus::RUNNING };
    let mut renderer = render::Renderer::new()?;
    let mut world = World::new();
    while state.status != GameStatus::EXIT {
        renderer.render_all(&world)?;
        control::handle_controls(&mut world, &mut state);
    }

    Ok(())
}