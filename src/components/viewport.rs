use crate::{
    commands,
    input::Key,
    renderer::{scroll_down, scroll_up, sync_scroll_area, RenderMsg},
};

#[derive(Clone, Debug)]
pub enum Message {
    Input(Key),
    SyncScrollArea(RenderMsg),
    ScrollDown(RenderMsg),
    ScrollUp(RenderMsg),
}

impl From<RenderMsg> for Message {
    fn from(msg: RenderMsg) -> Self {
        match msg {
            RenderMsg::SyncScrollArea { .. } => Message::SyncScrollArea(msg),
            RenderMsg::ScrollDown { .. } => Message::ScrollDown(msg),
            RenderMsg::ScrollUp { .. } => Message::ScrollUp(msg),
        }
    }
}

#[derive(Default, Clone)]
pub struct Model {
    pub width: isize,
    pub height: isize,
    pub y_offset: isize,
    pub y_position: isize,
    pub lines: Vec<String>,
}

impl Model {
    fn len(&self) -> isize {
        self.lines.len() as isize
    }

    pub fn at_top(&self) -> bool {
        self.y_offset <= 0
    }

    pub fn at_bottom(&self) -> bool {
        self.y_offset >= self.len() - 1 - self.height
    }

    pub fn past_bottom(&self) -> bool {
        self.y_offset > self.len() - 1 - self.height
    }

    pub fn scroll_percent(&self) -> f64 {
        if self.height >= self.len() {
            return 1.0;
        }

        let y = self.y_offset as f64;
        let h = self.height as f64;
        let t = (self.len() - 1) as f64;
        let v = y / (t - h);
        f64::max(0.0, f64::min(1.0, v))
    }

    pub fn set_content(&mut self, mut s: String) {
        s = s.replace("\r\n", "\n");
        self.lines = s.split("\n").map(|s| s.to_string()).collect();

        if self.y_offset > self.len() - 1 {
            self.goto_bottom();
        }
    }

    fn visible_lines(&self) -> Vec<String> {
        let mut lines = vec![];
        if self.len() > 0 {
            let top = max(0, self.y_offset);
            let bottom = clamp(self.y_offset + self.height, top, self.len());
            lines = self.lines[(top as usize)..(bottom as usize)].to_vec();
        }
        lines
    }

    fn view_down(&mut self) -> Option<Vec<String>> {
        if self.at_bottom() {
            return None;
        }

        self.y_offset = min(self.y_offset + self.height, self.len() - 1 - self.height);

        Some(self.visible_lines())
    }

    fn view_up(&mut self) -> Option<Vec<String>> {
        if self.at_top() {
            return None;
        }

        self.y_offset = max(self.y_offset - self.height, 0);

        Some(self.visible_lines())
    }

    fn half_view_down(&mut self) -> Option<Vec<String>> {
        let mut lines = vec![];

        if self.at_bottom() {
            return None;
        }

        self.y_offset = min(
            self.y_offset + self.height / 2,
            self.len() - 1 - self.height,
        );

        if self.len() > 0 {
            let top = max(self.y_offset + self.height / 2, 0);
            let bottom = clamp(self.y_offset + self.height, top, self.len() - 1);
            lines = self.lines[(top as usize)..(bottom as usize)].to_vec();
        }

        Some(lines)
    }

    fn half_view_up(&mut self) -> Option<Vec<String>> {
        let mut lines = vec![];

        if self.at_top() {
            return None;
        }

        self.y_offset = max(self.y_offset - self.height / 2, 0);

        if self.len() > 0 {
            let top = max(self.y_offset, 0);
            let bottom = clamp(self.y_offset + self.height / 2, top, self.len() - 1);
            lines = self.lines[(top as usize)..(bottom as usize)].to_vec();
        }

        Some(lines)
    }

    fn line_down(&mut self, mut n: isize) -> Option<Vec<String>> {
        let mut lines = vec![];

        if self.at_bottom() || n == 0 {
            return None;
        }

        let len = self.len();
        let max_delta = (len - 1) - (self.y_offset + self.height);
        n = min(n, max_delta);

        self.y_offset = min(self.y_offset + n, len - 1 - self.height);

        if len > 0 {
            let top = max(self.y_offset + self.height - n, 0);
            let bottom = clamp(self.y_offset + self.height, top, len - 1);
            lines = self.lines[(top as usize)..(bottom as usize)].to_vec();
        }

        Some(lines)
    }

    fn line_up(&mut self, mut n: isize) -> Option<Vec<String>> {
        let mut lines = vec![];

        if self.at_top() || n == 0 {
            return None;
        }

        n = min(n, self.y_offset);

        self.y_offset = max(self.y_offset - n, 0);

        let len = self.len();
        if len > 0 {
            let top = max(0, self.y_offset);
            let bottom = clamp(self.y_offset + n, top, len - 1);
            lines = self.lines[(top as usize)..(bottom as usize)].to_vec();
        }

        Some(lines)
    }

    fn goto_top(&mut self) -> Option<Vec<String>> {
        let mut lines = vec![];

        if self.at_top() {
            return None;
        }

        self.y_offset = 0;

        if self.len() > 0 {
            let top = self.y_offset;
            let bottom = clamp(self.y_offset + self.height, top, self.len() - 1);
            lines = self.lines[(top as usize)..(bottom as usize)].to_vec();
        }

        Some(lines)
    }

    fn goto_bottom(&mut self) -> Option<Vec<String>> {
        let mut lines = vec![];

        let len = self.len();
        self.y_offset = max(len - 1 - self.height, 0);

        if len > 0 {
            let top = self.y_offset;
            let bottom = max(len - 1, 0);
            lines = self.lines[(top as usize)..(bottom as usize)].to_vec();
        }

        Some(lines)
    }
}

pub fn input(event: Key) -> Option<Message> {
    Some(Message::Input(event))
}

pub fn update(msg: &Message, model: &mut Model) {
    match msg {
        Message::Input(key) => match key {
            Key::PageDown | Key::Char(' ') | Key::Char('f') => {
                model.view_down();
            }
            Key::PageUp | Key::Char('b') => {
                model.view_up();
            }
            Key::Char('d') | Key::Ctrl('d') => {
                model.half_view_down();
            }
            Key::Char('u') | Key::Ctrl('u') => {
                model.half_view_up();
            }
            Key::Down | Key::Char('j') => {
                model.line_down(1);
            }
            Key::Up | Key::Char('k') => {
                model.line_up(1);
            }
            _ => {}
        },
        Message::ScrollUp(_) => {
            model.line_up(1);
        }
        Message::ScrollDown(_) => {
            model.line_down(1);
        }
        _ => {}
    }
}

pub fn view(model: &Model) -> String {
    let lines = model.visible_lines();

    let mut extra_lines = String::from("");
    let len = lines.len() as isize;
    if len < model.height {
        extra_lines = "\n".repeat((model.height - len) as usize);
    }

    lines.join("\n") + &extra_lines
}

// COMMANDS

pub fn sync(m: &Model) -> Option<impl Fn() -> Message> {
    if m.lines.len() == 0 {
        return None;
    }

    let top = m.y_offset.max(0) as usize;
    let bottom = clamp(m.y_offset - m.height, 0, m.lines.len() as isize - 1);

    Some(commands::map(sync_scroll_area(
        m.lines[(top as usize)..(bottom as usize)].to_vec(),
        m.y_position,
        m.y_position + m.height,
    )))
}

pub fn view_down(m: &Model, lines: Vec<String>) -> Option<impl Fn() -> Message> {
    if lines.len() == 0 {
        return None;
    }

    Some(commands::map(scroll_down(
        lines,
        m.y_position,
        m.y_position + m.height,
    )))
}

pub fn view_up(m: &Model, lines: Vec<String>) -> Option<impl Fn() -> Message> {
    if lines.len() == 0 {
        return None;
    }

    Some(commands::map(scroll_up(
        lines,
        m.y_position,
        m.y_position + m.height,
    )))
}

fn clamp(v: isize, low: isize, high: isize) -> isize {
    min(high, max(low, v))
}

fn min(a: isize, b: isize) -> isize {
    if a < b {
        return a;
    }

    b
}

fn max(a: isize, b: isize) -> isize {
    if a > b {
        return a;
    }

    b
}
