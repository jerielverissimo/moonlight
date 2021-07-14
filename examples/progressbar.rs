use std::io::Result;
use std::{thread, time::Duration};

use moonlight::heartbeat::Heartbeat;
use moonlight::{input::InputEvent, BatchCmd, Cmd};

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
    fn update(self, msg: Msg) -> Self {
        let model = match msg {
            Msg::Frame => {
                let mut model = Model { ..self }; // deep copy
                if !model.loaded {
                    model.frames += 1;
                    //model.progress = ease_out_bounce(model.frames as f64 / 100.);
                    model.progress = model.frames as f64 / 200.;
                    if model.progress > 1. {
                        model.progress = 1.;
                        model.loaded = true;
                        model.ticks = 3;
                    }
                }
                model
            }
            Msg::Tick => {
                let mut model = Model { ..self }; // deepy copy
                if model.loaded {
                    model.ticks -= 1;
                    if model.ticks == 0 {
                        Heartbeat::stop();
                    }
                }
                model
            }
        };

        model
    }

    fn view(&self) -> String {
        let mut label = String::from("Downloading...");
        if self.loaded {
            label = format!("Downloaded. Exiting in {}...", self.ticks);
        }

        "\n".to_string() + label.as_str() + "\n" + progressbar(80, self.progress).as_str() + "%"
    }
}

#[derive(Clone)]
enum Msg {
    Frame,
    Tick,
}
fn reducer(model: Model, msg: Msg) -> (Model, BatchCmd<Msg>) {
    (model.update(msg), vec![])
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
    moonlight::Runtime::new(reducer, initialize, input, view)
        .with_fullscreen()
        .with_subscription(tick)
        .with_subscription(frame)
        .run()
}
