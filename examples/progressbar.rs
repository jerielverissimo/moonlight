use std::io::Result;
use std::{thread, time::Duration};

use moonlight::{
    input::InputEvent,
    quit,
    renderer::{exit_fullscreen, fullscreen},
    BatchCmd, Cmd, Sub,
};

pub fn ease_out_bounce(t: f64) -> f64 {
    if t < 4. / 11.0 {
        return (121. * t * t) / 16.0;
    } else if t < 8. / 11.0 {
        return (363. / 40.0 * t * t) - (99. / 10.0 * t) + 17. / 5.0;
    } else if t < 9. / 10.0 {
        return (4356. / 361.0 * t * t) - (35442. / 1805.0 * t) + 16061. / 1805.0;
    } else {
        return (54. / 5.0 * t * t) - (513. / 25.0 * t) + 268. / 25.0;
    }
}

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
                    //self.progress = ease_out_bounce(self.frames as f64 / 100.);
                    self.progress = self.frames as f64 / 200.;
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

fn input(event: InputEvent) -> Option<Msg> {
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

fn initialize() -> (Model, Option<Cmd<Msg>>) {
    let model = Model {
        ticks: 10,
        frames: 0,
        progress: 0.0,
        loaded: false,
    };
    (model, None)
}

fn main() -> Result<()> {
    fullscreen();
    let subs: Vec<Sub<Model, Msg>> = vec![Box::new(tick), Box::new(frame)];
    moonlight::program(initialize, update, view, input, subs)?;
    exit_fullscreen();
    Ok(())
}
