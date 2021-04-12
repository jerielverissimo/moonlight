use std::{thread, time::Duration};

use moonlight::{quit, BatchCmd, Sub};
use termion::event::Key;

#[derive(Clone)]
struct Model {
    loaded: bool,
    frames: i32,
    progress: f64,
    ticks: i32,
}

impl Model {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Frame => {
                if !self.loaded {
                    self.frames += 1;
                    self.progress = self.frames as f64 / 120.;
                    if self.progress > 1. {
                        self.progress = 1.;
                        self.loaded = true;
                        self.ticks = 3;
                    }
                }
            }
            Msg::Tick => {
                if self.loaded {
                    self.ticks -= 1;
                    if self.ticks == 0 {
                        quit();
                    }
                }
            }
        }
    }

    fn view(&self) -> String {
        let mut label = String::from("Downloading...");
        if self.loaded {
            label = format!("Downloaded. Exiting in {}...", self.ticks);
        }

        "\n".to_string() + label.as_str() + "\n" + progressbar(80, self.progress).as_str() + "%"
    }
}

enum Msg {
    Frame,
    Tick,
}

fn update(msg: Msg, model: &mut Model) -> BatchCmd<Msg> {
    model.update(msg);
    vec![]
}

fn view(model: &Model) -> String {
    model.view()
}

fn input(event: Key) -> Option<Msg> {
    match event {
        _ => None,
    }
}

// Progressbar widget
fn progressbar(width: i32, percent: f64) -> String {
    let meta_chars = 7;
    let w = (width - meta_chars) as f64;
    let full_size = (f64::round(w * percent)) as i32;
    let empty_size = w as i32 - full_size;
    let full_cells = "#".repeat(full_size as usize);
    let empty_cells = ".".repeat(empty_size as usize);
    return format!(
        "[{}{}] {}",
        full_cells,
        empty_cells,
        f64::round(percent * 100.),
    );
}

fn frame(_: &Model) -> Msg {
    thread::sleep(Duration::from_millis(16));
    Msg::Frame
}

fn tick(_: &Model) -> Msg {
    thread::sleep(Duration::from_secs(1));
    Msg::Tick
}

fn main() {
    let model = Model {
        ticks: 10,
        frames: 0,
        progress: 0.0,
        loaded: false,
    };
    let subs: Vec<Sub<Model, Msg>> = vec![Box::new(tick), Box::new(frame)];
    moonlight::program(model, update, view, input, subs);
}
