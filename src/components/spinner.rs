use std::time::Duration;

use crate::commands;
use crate::BatchCmd;

pub enum SpinnerType {
    Line,
    Dot,
    MiniDot,
    Jump,
    Pulse,
    Points,
    Globe,
    Moon,
    Monkey,
}

/// Spinner is a set of frames used in animating the spinner.
#[derive(Default, Clone)]
pub struct Spinner {
    frames: Vec<&'static str>,
    fps: Duration,
}

impl Spinner {
    /// Some spinners to choose from. You could also make your own.
    pub fn new(spinner_type: SpinnerType) -> Spinner {
        match spinner_type {
            SpinnerType::Line => Spinner {
                frames: vec!["|", "/", "-", "\\"],
                fps: Duration::from_secs(1) / 10,
            },
            SpinnerType::Dot => Spinner {
                frames: vec!["⣾ ", "⣽ ", "⣻ ", "⢿ ", "⡿ ", "⣟ ", "⣯ ", "⣷ "],
                fps: Duration::from_secs(1) / 10,
            },
            SpinnerType::MiniDot => Spinner {
                frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                fps: Duration::from_secs(1) / 12,
            },
            SpinnerType::Jump => Spinner {
                frames: vec!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
                fps: Duration::from_secs(1) / 10,
            },
            SpinnerType::Pulse => Spinner {
                frames: vec!["█", "▓", "▒", "░"],
                fps: Duration::from_secs(1) / 8,
            },
            SpinnerType::Points => Spinner {
                frames: vec!["∙∙∙", "●∙∙", "∙●∙", "∙∙●"],
                fps: Duration::from_secs(1) / 7,
            },
            SpinnerType::Globe => Spinner {
                frames: vec!["🌍", "🌎", "🌏"],
                fps: Duration::from_secs(1) / 4,
            },
            SpinnerType::Moon => Spinner {
                frames: vec!["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
                fps: Duration::from_secs(1) / 8,
            },
            SpinnerType::Monkey => Spinner {
                frames: vec!["🙈", "🙉", "🙊"],
                fps: Duration::from_secs(1) / 3,
            },
        }
    }
}

/// TickMsg indicates that the timer has ticked and we should render a frame.
pub struct TickMsg {
    tag: i32,
}

/// Model contains the state for the spinner. Use new to create new models
/// rather than using Model as a struct literal.
#[derive(Default, Clone)]
pub struct Model {
    /// Spinner settings to use. See type Spinner.
    spinner: Spinner,
    frame: usize,
    tag: i32,
}

impl Model {
    /// NewModel returns a model with default values.
    pub fn new() -> Self {
        Self {
            spinner: Spinner::new(SpinnerType::Line),
            ..Default::default()
        }
    }

    pub fn with(spinner_type: SpinnerType) -> Self {
        Self {
            spinner: Spinner::new(spinner_type),
            ..Default::default()
        }
    }

    fn tick(&self, tag: i32) -> BatchCmd<TickMsg> {
        let cmd = commands::tick(self.spinner.fps, move || TickMsg { tag });
        vec![cmd]
    }
}

// Update is the Moonlight update function. This will advance the spinner one frame
// every time it's called.
pub fn update(msg: TickMsg, model: &mut Model) -> BatchCmd<TickMsg> {
    if msg.tag > 0 && msg.tag != model.tag {
        return vec![];
    }

    model.frame += 1;

    if model.frame >= model.spinner.frames.len() {
        model.frame = 0;
    }

    model.tag += 1;

    model.tick(model.tag)
}

/// View renders the model's view.
pub fn view(model: &Model) -> String {
    if model.frame >= model.spinner.frames.len() {
        return "(error)".into();
    }

    model.spinner.frames[model.frame].into()
}

/// Tick is the command used to advance the spinner one frame. Use this command
/// to effectively start the spinner.
pub fn tick() -> TickMsg {
    TickMsg { tag: 0 }
}
