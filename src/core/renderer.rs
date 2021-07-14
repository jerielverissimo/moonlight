use std::io::{stdout, Result, Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};

use crate::Cmd;

const ESC: &str = "\x1B[";

static mut LINES_RENDERED: i32 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderMsg {
    SyncScrollArea {
        lines: Vec<String>,
        top_boundary: isize,
        bottom_boundary: isize,
    },
    ScrollDown {
        lines: Vec<String>,
        top_boundary: isize,
        bottom_boundary: isize,
    },
    ScrollUp {
        lines: Vec<String>,
        top_boundary: isize,
        bottom_boundary: isize,
    },
}

pub struct Renderer {
    stdout: RawTerminal<Stdout>,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        Self { stdout }
    }

    pub(crate) fn render(&mut self, view: &str) -> Result<()> {
        let mut view = view.to_owned() + "\n";

        // We need to add carriage returns to ensure that the cursor travels to the
        // start of a column after a newline
        view = view.replace("\n", "\r\n");

        unsafe {
            if LINES_RENDERED > 0 {
                self.clear_lines(LINES_RENDERED)?;
            }
            LINES_RENDERED = 0;
        }

        write!(self.stdout, "{}", view)?; // render
        self.stdout.flush().unwrap();

        unsafe {
            LINES_RENDERED = view.matches("\r\n").count() as i32;
        }
        Ok(())
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        write!(self.stdout, "{}", termion::cursor::Hide)?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        write!(self.stdout, "{}", termion::cursor::Show)?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    pub fn restore_terminal(&mut self) -> Result<()> {
        write!(self.stdout, "{}", termion::cursor::Restore)?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    // Move the cursor up a given number of lines and place it at the beginning of
    // the line
    fn cursor_prev_line(&mut self, n: i32) -> Result<()> {
        write!(self.stdout, "{}{}F", ESC.to_owned(), n)?;
        //self.stdout.flush().unwrap();
        Ok(())
    }

    fn clear_line(&mut self) -> Result<()> {
        write!(self.stdout, "{}", ESC.to_owned() + "2K")?;
        //self.stdout.flush().unwrap();
        Ok(())
    }

    fn clear_lines(&mut self, n: i32) -> Result<()> {
        for _ in 0..n {
            self.cursor_prev_line(1)?;
            self.clear_line()?;
        }
        Ok(())
    }
}

// invert inverts the foreground and background colors of a given string
pub fn invert(s: String) -> String {
    ESC.to_owned() + "7m" + &s + ESC + "0m"
}

// fullscreen switches to the altscreen and clears the terminal. The former
// view can be restored with exit_fullscreen().
pub fn fullscreen() {
    print!("{}", ESC.to_owned() + "?1049h" + ESC + "H");
}

// exit_fullscreen exits the altscreen and returns the former terminal view
pub fn exit_fullscreen() {
    print!("{}", ESC.to_owned() + "?1049l");
}

// HIGH-PERFORMANCE RENDERING STUFF

pub fn sync_scroll_area(
    lines: Vec<String>,
    top_boundary: isize,
    bottom_boundary: isize,
) -> Cmd<RenderMsg> {
    Box::new(move || RenderMsg::SyncScrollArea {
        lines: lines.clone(),
        top_boundary,
        bottom_boundary,
    })
}

pub fn scroll_down(
    new_lines: Vec<String>,
    top_boundary: isize,
    bottom_boundary: isize,
) -> Cmd<RenderMsg> {
    Box::new(move || RenderMsg::ScrollDown {
        lines: new_lines.clone(),
        top_boundary,
        bottom_boundary,
    })
}

pub fn scroll_up(
    new_lines: Vec<String>,
    top_boundary: isize,
    bottom_boundary: isize,
) -> Cmd<RenderMsg> {
    Box::new(move || RenderMsg::ScrollUp {
        lines: new_lines.clone(),
        top_boundary,
        bottom_boundary,
    })
}
