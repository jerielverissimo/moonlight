const ESC: &str = "\x1B[";

static mut LINES_RENDERED: i32 = 0;

pub(crate) fn render(view: &str) {
    let mut view = view.to_owned() + "\n";

    // We need to add carriage returns to ensure that the cursor travels to the
    // start of a column after a newline
    view = view.replace("\n", "\r\n");

    unsafe {
        if LINES_RENDERED > 0 {
            clear_lines(LINES_RENDERED);
        }
    }

    print!("{}", view); // render

    unsafe {
        LINES_RENDERED = view.matches("\r\n").count() as i32;
    }
}

pub fn hide_cursor() {
    print!("{}", termion::cursor::Hide);
}

pub fn show_cursor() {
    print!("{}", termion::cursor::Show);
}

pub fn restore_terminal() {
    print!("{}", termion::cursor::Restore);
}

// Move the cursor up a given number of lines and place it at the beginning of
// the line
fn cursor_prev_line(n: i32) {
    print!("{}{}F", ESC.to_owned(), n);
}

fn clear_line() {
    print!("{}", ESC.to_owned() + "2K");
}

fn clear_lines(n: i32) {
    for _ in 0..n {
        cursor_prev_line(1);
        clear_line();
    }
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
