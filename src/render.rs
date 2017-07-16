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

use std::io::{stdout, Write };
use std::io;

use core::World;
use core::TileType;

use termion::{color, cursor, clear};
use termion::raw::{RawTerminal, IntoRawMode};

pub struct Renderer {
    stdout: RawTerminal<io::Stdout>,
}

impl Renderer {
    pub fn new() -> io::Result<Renderer> {
        let stdout = (stdout().into_raw_mode())?;
        Ok(Renderer { stdout })
    }

    fn draw_char<C1, C2>(&mut self, ch: char, x: u16, y: u16, bg: C1, fg: C2) -> io::Result<()>
        where C1: color::Color, C2: color::Color {
        let stdout = &mut self.stdout;
        write!(stdout, "{position}{bg}{fg}{symbol}{hide}{reset_bg}{reset_fg}",
               position = cursor::Goto(x + 1, y + 1),
               bg = color::Bg(bg),
               fg = color::Fg(fg),
               symbol = ch,
               hide = cursor::Hide,
               reset_bg = color::Bg(color::Reset),
               reset_fg = color::Fg(color::Reset))
    }

    /// Renders to TTY the whole world with creatures, items, etc.
    pub fn render_all(&mut self, world: &World) -> io::Result<()> {
        write!(self.stdout, "{}", clear::All)?;

        for (x, y, tile) in world.current_tiles() {
            let (bg, symbol) = match tile.tile_type {
                TileType::Wall => (color::AnsiValue::rgb(3, 3, 3), ' '),
                TileType::Ground => (color::AnsiValue::rgb(0, 0, 0), ' '),
                _ => unimplemented!() // TODO: Add another tile types for rendering
            };
            self.draw_char(symbol, x as u16, y as u16, bg, color::White)?;
        }

        self.stdout.flush()?;

        Ok(())
    }
}

impl Drop for Renderer {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        write!(self.stdout, "{}", clear::All);
        write!(self.stdout, "{}{}", color::Fg(color::Reset), color::Bg(color::Reset));
        write!(self.stdout, "{}", cursor::Show);
    }
}



