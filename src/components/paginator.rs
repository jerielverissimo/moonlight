use termion::color;
use termion::event::Key;

use crate::color::convert_hex_rgb;

/// paginator module provides a Moonlight module for calculating pagination and
/// rendering pagination info. Note that this package does not render actual
/// pages: it's purely for handling keystrokes related to pagination, and
/// rendering pagination status.

/// PaginatorType specifies the way we render pagination
#[derive(Clone)]
pub enum PaginatorType {
    Arabic,
    Dots,
}

const BRIGHT_GRAY: &'static str = "#DDDADA";
const DARK_GRAY: &'static str = "#847A85";

/// Model is the Moonlight model for this user interface
#[derive(Clone)]
pub struct Model {
    paginator_type: PaginatorType,
    page: i32,
    per_page: i32,
    total_pages: i32,
    active_dot: String,
    inactive_dot: String,
    use_left_right_keys: bool,
    use_up_down_keys: bool,
    use_h_l_keys: bool,
    use_j_k_keys: bool,
}

impl Model {
    /// new creates a new model with defaults
    pub fn new() -> Self {
        Self {
            paginator_type: PaginatorType::Dots,
            page: 0,
            per_page: 1,
            total_pages: 1,
            active_dot: format!(
                "{}{}",
                "•",
                color::Fg(convert_hex_rgb(BRIGHT_GRAY).unwrap()),
            ),
            inactive_dot: format!("{}{}", "•", color::Fg(convert_hex_rgb(DARK_GRAY).unwrap()),),
            use_left_right_keys: true,
            use_up_down_keys: false,
            use_h_l_keys: true,
            use_j_k_keys: false,
        }
    }

    /// set_total_pages is a helper function for calculating the total number of pages
    /// from a given number of items. It's use is optional since this pager can be
    /// used for other things beyond navigating sets. Note that it both returns the
    /// number of total pages and alters the model.
    pub fn set_total_pages(&mut self, items: i32) -> i32 {
        if items < 1 {
            return self.total_pages;
        }

        let mut n = items / self.per_page;
        if items % self.per_page > 0 {
            n += 1;
        }

        self.total_pages = n;

        n
    }

    pub fn paginator_type(&mut self, paginator_type: PaginatorType) {
        self.paginator_type = paginator_type;
    }

    /// items_on_page is a helper function for returning the number of items on the
    /// current page given the total number of items passed as an argument.
    pub fn items_on_page(&self, total_items: i32) -> i32 {
        if total_items < 1 {
            return 0;
        }

        let (start, end) = self.slice_bounds(total_items);
        end - start
    }

    /// slice_bounds is a helper function for paginating slices. Pass the length
    /// of the slice you're rendering and you'll receive the start and end bounds
    /// corresponding the to pagination. For example:
    ///
    ///     let bunch_of_stuff = stuff {...}
    ///     let (start, end) = model.get_slice_bounds(bunch_of_stuff.len());
    ///     slice_to_render = bunch_of_stuff[start..end]
    pub fn slice_bounds(&self, length: i32) -> (i32, i32) {
        let start = self.page * self.per_page;
        let end = min(self.page * self.per_page + self.per_page, length);
        (start, end)
    }

    /// prev_page is a number function for navigating one page backward. It will not
    /// page beyond the first page (i.e. page 0).
    pub fn prev_page(&mut self) {
        if self.page > 0 {
            self.page -= 1;
        }
    }

    /// next_page is a helper function for navigating one page forward. It will not
    /// page beyond the last page (i.e. totalPages - 1).
    pub fn next_page(&mut self) {
        if !self.on_last_page() {
            self.page += 1;
        }
    }

    /// on_last_page returns whether or not we're on the last page
    pub fn on_last_page(&self) -> bool {
        self.page == self.total_pages - 1
    }
}

// input is the Moonlight input function which binds keystrokes to pagination
pub fn input<MSG>(m: &mut Model, event: Key) -> Option<MSG> {
    if m.use_left_right_keys {
        match event {
            Key::Left => m.prev_page(),
            Key::Right => m.next_page(),
            _ => return None,
        }
    }

    if m.use_up_down_keys {
        match event {
            Key::Up => m.prev_page(),
            Key::Down => m.next_page(),
            _ => return None,
        }
    }

    if m.use_h_l_keys {
        match event {
            Key::Char('h') => m.prev_page(),
            Key::Char('l') => m.next_page(),
            _ => return None,
        }
    }

    if m.use_j_k_keys {
        match event {
            Key::Char('j') => m.prev_page(),
            Key::Char('k') => m.next_page(),
            _ => return None,
        }
    }

    None
}

/// view renders the pagination to a string
pub fn view(model: &Model) -> String {
    match &model.paginator_type {
        PaginatorType::Dots => dots_view(model),
        _ => arabic_view(model),
    }
}

fn dots_view(model: &Model) -> String {
    let mut s = String::new();

    for i in 0..model.total_pages {
        if i == model.page {
            s += model.active_dot.as_str();
            continue;
        }
        s += model.inactive_dot.as_str();
    }

    s
}

fn arabic_view(model: &Model) -> String {
    format!("{}/{}", model.page + 1, model.total_pages)
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }

    b
}
