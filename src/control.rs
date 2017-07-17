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
use std::io::{stdin, stdout, Write};

use core::World;
use super::GameState;
use super::GameStatus;

use termion::event::Event;
use termion::input::TermRead;
use termion::event::Key;

pub fn handle_controls(world: &mut World, game_state: &mut GameState) {
    let stdin = stdin();
    stdout().flush().unwrap();
    if let Some(Ok(event)) = stdin.events().nth(0) {
        match event {
            Event::Key(key) => handle_key(world, game_state, key),
            _ => (),
        }
    }
    // stub
}

fn handle_key(world: &mut World, game_state: &mut GameState, key: Key) {
    match key {
        Key::Esc => game_state.status = GameStatus::EXIT,
        _ => ()
    }
}