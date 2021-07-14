use std::{thread, time::Duration};

use termion::color;

use crate::{convert_hex_rgb, renderer::invert, Cmd, Key};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    CursorBlink,
    Key(Key),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EchoMode {
    Normal,
    Password,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CursorMode {
    Blink,
    Static,
    Hide,
}

#[derive(Clone)]
pub struct Model {
    pub prompt: String,
    pub placeholder: String,
    pub blink_speed: Duration,
    pub value: String,
    pub cursor: String,
    text_color: String,
    placeholder_color: String,
    cursor_color: String,
    cusrsor_mode: CursorMode,

    pub char_limit: usize,
    focus: bool,
    blink: bool,
    pos: usize,
}

impl Model {
    pub fn new() -> Self {
        Self {
            prompt: String::from("> "),
            value: String::new(),
            cursor: String::from("\u{2588}"),
            blink_speed: Duration::from_millis(600),
            placeholder: String::new(),
            text_color: String::new(),
            placeholder_color: String::new(),
            cursor_color: String::new(),
            cusrsor_mode: CursorMode::Blink,
            char_limit: 0,
            focus: false,
            blink: true,
            pos: 0,
        }
    }

    pub fn blink(&self) -> Message {
        thread::sleep(self.blink_speed);
        Message::CursorBlink
    }

    pub fn focused(&self) -> bool {
        self.focus
    }

    pub fn focus(&mut self) -> Option<Cmd<Message>> {
        self.focus = true;
        self.blink = self.cusrsor_mode == CursorMode::Hide;

        if self.cusrsor_mode == CursorMode::Blink && self.blink {
            return self.blink_cmd();
        }

        None
    }

    pub fn blur(&mut self) {
        self.focus = false;
        self.blink = true;
    }

    fn color_text(&self, s: &str) -> String {
        if self.text_color.is_empty() {
            return String::from(s);
        }

        format!(
            "{}{}",
            s,
            color::Fg(convert_hex_rgb(&self.text_color).unwrap())
        )
    }

    pub fn color_placeholder(&self, s: &str) -> String {
        if self.placeholder_color.is_empty() {
            return String::from(s);
        }

        format!(
            "{}{}",
            s,
            color::Fg(convert_hex_rgb(&self.placeholder_color).unwrap())
        )
    }

    pub fn view(&self) -> String {
        if self.value.is_empty() && !self.placeholder.is_empty() {
            return placeholder_view(self);
        }

        let mut v = self.color_text(&self.value[..self.pos]);

        if self.pos < self.value.len() {
            v += &cursor_view(&self.value.chars().nth(self.pos).unwrap().to_string(), self);
            v += &self.color_text(&self.value[self.pos + 1..]);
        } else {
            v += &cursor_view(" ", self);
        }

        self.prompt.clone() + &v
    }

    pub fn reducer(&mut self, msg: Message) {
        if !self.focus {
            self.blink = true;
        }

        match msg {
            Message::CursorBlink => {
                self.blink = !self.blink;
            }
            Message::Key(k) => match k {
                Key::Backspace => {
                    if !self.value.is_empty() {
                        self.value = self.value[..max(0, self.pos - 1)].to_string()
                            + &self.value[self.pos..];
                        if self.pos > 0 {
                            self.pos -= 1;
                        }
                    }
                }
                Key::Delete => {
                    if self.value.is_empty() || self.pos >= self.value.len() {
                        return;
                    }

                    self.value =
                        self.value[..max(0, self.pos)].to_string() + &self.value[self.pos..];
                }
                Key::Left => {
                    if self.pos > 0 {
                        self.pos -= 1;
                    }
                }
                Key::Right => {
                    if self.pos < self.value.len() {
                        self.pos += 1;
                    }
                }
                Key::Ctrl('f') | Key::Ctrl('b') | Key::Ctrl('a') => {
                    self.pos = 0;
                }
                Key::Ctrl('d') => {
                    if !self.value.is_empty() && self.pos < self.value.len() {
                        self.value =
                            self.value[..self.pos].to_string() + &self.value[self.pos + 1..];
                    }
                }
                Key::Ctrl('e') => {
                    self.pos = self.value.len();
                }
                Key::Ctrl('k') => {
                    self.value = self.value[..self.pos].to_string();
                    self.pos = self.value.len();
                }
                Key::Ctrl('u') => {
                    self.value = self.value[self.pos..].to_string();
                    self.pos = 0;
                }
                Key::Char(c) => {
                    if self.char_limit == 0 || self.value.len() < self.char_limit {
                        self.value = self.value[..self.pos].to_string()
                            + &c.to_string()
                            + &self.value[self.pos..];
                        self.pos += 1;
                    }
                }
                _ => {}
            },
        }
    }

    fn blink_cmd(&mut self) -> Option<Cmd<Message>> {
        if self.cusrsor_mode == CursorMode::Blink {
            return None;
        }

        Some(Box::new(|| Message::CursorBlink))
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

fn placeholder_view(model: &Model) -> String {
    let mut v = String::new();
    let p = &model.placeholder;

    if model.blink && !model.placeholder.is_empty() {
        v += &cursor_view(&model.color_placeholder(&p[..1]), model);
    } else {
        v += &cursor_view(&p[..1], model);
    }

    v += &model.color_placeholder(&p[1..]);

    model.prompt.clone() + &v
}

fn cursor_view(s: &str, model: &Model) -> String {
    if model.blink {
        return s.to_string();
    }

    invert(s.to_string())
}

fn max(a: usize, b: usize) -> usize {
    if a > b {
        return a;
    }

    b
}
