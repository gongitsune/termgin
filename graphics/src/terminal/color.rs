use super::Terminal;
use crate::render::texture::Texture;
use glam::Vec4;
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::io::{stdout, BufWriter, StdoutLock, Write};

pub struct ColorTerminal<'a> {
    out: BufWriter<StdoutLock<'a>>,
    stream: String,
    width: usize,
    height: usize,
}

impl<'a> ColorTerminal<'a> {
    pub fn new() -> Self {
        let mut ws: winsize = unsafe { std::mem::zeroed() };
        ws = if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) } == -1 {
            None
        } else {
            Some(ws)
        }
        .unwrap();

        let width = ws.ws_col as usize;
        let height = ws.ws_row as usize;
        let stream = String::with_capacity(width * height * 16);
        Self {
            out: BufWriter::new(stdout().lock()),
            stream,
            width,
            height,
        }
    }

    fn write_pixel(&mut self, bg_color: &Vec4, fg_color: &Vec4) {
        // background
        {
            let r = (bg_color.x * 255.) as usize;
            let g = (bg_color.y * 255.) as usize;
            let b = (bg_color.z * 255.) as usize;
            self.stream
                .push_str(&format!("\x1b[48;2;{};{};{}m", r, g, b));
        }

        // foreground
        {
            let r = (fg_color.x * 255.) as usize;
            let g = (fg_color.y * 255.) as usize;
            let b = (fg_color.z * 255.) as usize;
            self.stream
                .push_str(&format!("\x1b[38;2;{};{};{}m", r, g, b));
        }

        // character
        self.stream.push('\u{2584}');
    }
}

impl<'a> Terminal for ColorTerminal<'a> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn present(&mut self, tex: &Texture) {
        self.stream.clear();

        let mut bg_color = Vec4::ZERO;
        let mut fg_color = Vec4::ZERO;
        for y in (0..self.height).step_by(2) {
            for x in 0..self.width {
                tex.get_to_out(x, y % self.height, &mut bg_color);
                tex.get_to_out(x, (y + 1) % self.height, &mut fg_color);
                self.write_pixel(&bg_color, &fg_color);
            }
            self.stream.push('\n');
        }
        self.stream.push_str("\x1b[0m");

        self.out.write_all(self.stream.as_bytes()).unwrap();
    }
}
