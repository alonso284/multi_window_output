//! multi-window-output is a tool for allowing multiple output windows in the same screen. You can
//! have as many `Screen`s as you please; however, everytime you call `Screen::flush()` or
//! `Screen::println()`, the current terminal screen will be replaced with the output of the calling screen.

mod window;
use std::io::Write;
use termion::color;
use termion::screen::IntoAlternateScreen;
use termion::terminal_size;
use window::{Priority, Window};

const MAX_WIN: usize = 6;
const INIT: Option<Window> = None;
const MAX_WIDTH: usize = 512;
const MAX_HEIGHT: usize = 254;

#[cfg(test)]
mod screen_tests {
    use super::*;
    #[test]
    fn create_screen() {
        let mut screen = Screen::new();
        let id_1 = screen.append_left_child(0).unwrap();
        let id_2 = screen.append_down_child(0).unwrap();
        screen.println(id_1, "Hello World").unwrap();
        screen.println(id_2, "Hello World").unwrap();
    }
    #[test]
    fn bridge_creation() {
        let mut screen = Screen::name_screen("My New Screen");
        screen.set_name("Change the name");
        screen.set_window_name(0, "My only window").unwrap();
        let bridge = Bridge::new(screen);
        bridge.println(0, "Hello World").unwrap();
    }
    #[test]
    fn not_found(){
        let mut screen = Screen::new();
        let err = screen.flush(3);
        assert_eq!(Err(std::io::ErrorKind::NotFound), err);
    }
    #[test]
    fn alredy_exists(){
        let mut screen = Screen::new();
        screen.append_left_child(0).unwrap();
        let err = screen.append_left_child(0);
        assert_eq!(Err(std::io::ErrorKind::AlreadyExists), err);
    }
    #[test]
    fn out_of_memory(){
        let mut screen = Screen::new();
        let mut id:usize = 0;
        for _ in 0..(MAX_WIN - 1) {
            id = screen.append_left_child(id).unwrap();
        }
        let err = screen.append_left_child(0);
        assert_eq!(Err(std::io::ErrorKind::OutOfMemory), err);
    }
}

#[derive(Debug)]
/// A `Screen` contains multiple windows. When initiated, a `Screen` only contains one window with
/// `id = 0`.
pub struct Screen {
    name: String,
    windows: [Option<Window>; MAX_WIN],
    count: usize,
    buffer: [[(u8, bool); MAX_WIDTH]; MAX_HEIGHT],
}

impl Screen {
    /// Create a new `Screen` with one window with `id = 0`.
    pub fn new() -> Screen {
        let mut screen = Screen {
            windows: [INIT; MAX_WIN],
            count: 1,
            name: "Screen".to_string(),
            buffer: [[(b' ', false); MAX_WIDTH]; MAX_HEIGHT],
        };
        screen.windows[0] = Some(Window::new(0));
        screen.load();
        screen
    }
    /// Create a new `Screen` with name
    pub fn name_screen(name: &str) -> Screen {
        let mut screen = Screen::new();
        screen.set_name(name);
        screen
    }
    /// Set name to `Screen`
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    /// Set name to window
    pub fn set_window_name(&mut self, id: usize, name: &str) -> Result<(), std::io::ErrorKind> {
        self.validate_id(id)?;
        self.windows[id].as_mut().unwrap().name = name.to_string();
        Ok(())
    }
    fn load(&mut self) {
        let mut scr = std::io::stdout().into_alternate_screen().unwrap();

        let (width, height) = terminal_size().unwrap();
        println!(
            "{}Screen: {}{}{}",
            color::Bg(color::Green),
            self.name,
            " ".repeat((width - 8 - self.name.len() as u16) as usize),
            color::Bg(color::Reset)
        );
        let width = width as usize;
        let height = height as usize - 1;

        self.output(0, 0, width, 0, height);

        for i in 0..height {
            for j in 0..width {
                if self.buffer[i][j].1 {
                    print!(
                        "{}{}{}",
                        color::Bg(color::Green),
                        self.buffer[i][j].0 as char,
                        color::Bg(color::Reset)
                    );
                } else {
                    print!("{}", self.buffer[i][j].0 as char);
                }
            }
        }
        scr.flush().unwrap();
    }
    fn output(
        &mut self,
        id: usize,
        start_width: usize,
        mut end_width: usize,
        start_height: usize,
        mut end_height: usize,
    ) {
        match self.windows[id].as_ref().unwrap().priority {
            Some(Priority::Vertical) => {
                let mit = (start_width + end_width) / 2;
                let left_id: usize = *self.windows[id]
                    .as_ref()
                    .unwrap()
                    .left_child
                    .as_ref()
                    .unwrap();
                self.output(left_id, mit, end_width, start_height, end_height);
                end_width = mit;
                if let Some(down_id) = self.windows[id].as_ref().unwrap().down_child.as_ref() {
                    let mit = (start_height + end_height) / 2;
                    self.output(*down_id, start_width, end_width, mit, end_height);
                    end_height = mit;
                }
            }
            Some(Priority::Horizontal) => {
                let mit = (start_height + end_height) / 2;
                let down_id: usize = *self.windows[id]
                    .as_ref()
                    .unwrap()
                    .down_child
                    .as_ref()
                    .unwrap();
                self.output(down_id, start_width, end_width, mit, end_height);
                end_height = mit;
                if let Some(left_id) = self.windows[id].as_ref().unwrap().left_child.as_ref() {
                    let mit = (start_width + end_width) / 2;
                    self.output(*left_id, mit, end_width, start_height, end_height);
                    end_width = mit;
                }
            }
            None => {}
        }

        let buffer_size = self.windows[id].as_ref().unwrap().buffer.iter().count();

        let mut it = self.windows[id].as_ref().unwrap().buffer.iter();
        if buffer_size > end_height - start_height - 1 {
            it.nth(buffer_size - (end_height - start_height))
                .unwrap()
                .as_ref()
                .unwrap();
        }

        let empty_line = "-- ".to_string();
        for i in start_height..end_height - 1 {
            let line = match it.next() {
                Some(s) => s.as_ref().unwrap(),
                None => &empty_line,
            };
            let mut letter = line.chars();
            for j in start_width..end_width {
                if j == end_width - 1 {
                    self.buffer[i][j] = (b' ', true)
                } else {
                    self.buffer[i][j] = (
                        match letter.next() {
                            Some(l) => l as u8,
                            None => b' ',
                        },
                        false,
                    )
                };
            }
        }

        // Put the name in the lower part
        let name = format!(
            "{} ID: {}",
            self.windows[id].as_ref().unwrap().get_name(),
            id
        );
        let mut c = name.chars();
        for j in start_width..end_width {
            self.buffer[end_height - 1][j] = (
                match c.next() {
                    Some(l) => l as u8,
                    None => b' ',
                },
                true,
            );
        }
    }
    // Validate existance of window
    fn validate_id(&self, id: usize) -> Result<(), std::io::ErrorKind> {
        // Invalid ID
        if id >= MAX_WIN || self.windows[id].is_none(){
            return Err(std::io::ErrorKind::NotFound);
        }
        Ok(())
    }
    /// Splits window with `id` into a left and right window. If successfull, the left window keeps the `id` you
    /// passed, and the function returns `Ok(id)`, with the `id` of the right window. If failed, returns
    /// `Err(std::io::ErrorKind)`.
    pub fn append_left_child(&mut self, id: usize) -> Result<usize, std::io::ErrorKind> {
        self.append_child(id, Priority::Vertical)
    }
    /// Splits window with `id` into an up and down window. If successfull, the up window keeps the `id` you
    /// passed, and the function returns `Ok(id)`, with the `id` of the down window. If failed, returns
    /// `Err(std::io::ErrorKind)`.
    pub fn append_down_child(&mut self, id: usize) -> Result<usize, std::io::ErrorKind> {
        self.append_child(id, Priority::Horizontal)
    }
    fn append_child(&mut self, id: usize, priority: Priority) -> Result<usize, std::io::ErrorKind> {
        // Not enough space to add new window
        if self.count >= MAX_WIN {
            return Err(std::io::ErrorKind::OutOfMemory);
        }
        // Validate if child exits
        self.validate_id(id)?;

        // Validate that node doesn't alredy have child
        let child = match priority {
            Priority::Vertical => self.windows[id].as_ref().unwrap().left_child,
            Priority::Horizontal => self.windows[id].as_ref().unwrap().down_child,
        };
        if child.is_some() {
            return Err(std::io::ErrorKind::AlreadyExists);
        }

        self.windows[self.count] = Some(Window::new(self.count));
        match priority {
            Priority::Vertical => self.windows[id].as_mut().unwrap().left_child = Some(self.count),
            Priority::Horizontal => {
                self.windows[id].as_mut().unwrap().down_child = Some(self.count)
            }
        };
        if self.windows[id].as_ref().unwrap().priority.is_none() {
            self.windows[id].as_mut().unwrap().priority = Some(priority);
        }
        self.count += 1;
        Ok(self.count - 1)
    }
    /// Print a new `line` in window with `id`. Returns `()` if successful, `Err(std::io::ErrorKind)` if not.
    pub fn println(&mut self, id: usize, line: &str) -> Result<(), std::io::ErrorKind> {
        // Validate if child exits
        self.validate_id(id)?;
        self.print(id, line).unwrap();
        self.flush(id)
    }
    /// Print `line` in window with `id`, but do not flush. Returns `()` if successful, `Err(std::io::ErrorKind)` if not.
    pub fn print(&mut self, id: usize, line: &str) -> Result<(), std::io::ErrorKind> {
        // Validate if child exits
        self.validate_id(id)?;
        self.windows[id].as_mut().unwrap().print(line);
        Ok(())
    }
    /// Flush window with `id`. Returns `()` if successful, `Err(std::io::ErrorKind)` if not.
    pub fn flush(&mut self, id: usize) -> Result<(), std::io::ErrorKind> {
        // Validate if child exits
        self.validate_id(id)?;
        self.windows[id].as_mut().unwrap().flush();
        self.load();
        Ok(())
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen::new()
    }
}
enum Cmds {
    Print,
    Flush,
    Println,
    Break,
}

/// `Bridge` is a complement for `Screen`. It allows you to print new lines in windows from different threads.
/// You initiate a `Bridge` by passing a pre-created `Screen`. The only disadvantage of using
/// `Bridge` is that you can't create new children once you have created the `Bridge`. Run `Bridge::clone(&self)` to
/// access bridge from multiple threads. Ideally, run `Bridge::kill(&self)`
/// before ending program to kill the screening process. However, it is not neccessary.
pub struct Bridge {
    bridge: std::sync::mpsc::Sender<(Cmds, usize, String)>,
    hash: std::collections::HashSet<usize>,
}

impl Bridge {
    /// Create a `Bridge` by passing an already created `Screen`. You won't be able to append children
    /// to `Screen` once passed.
    pub fn new(screen: Screen) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<(Cmds, usize, String)>();
        let mut hash: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let ids: Vec<usize> = screen
            .windows
            .iter()
            .map(|window| match &window {
                Some(window) => window.get_id(),
                None => 0,
            })
            .rev()
            .collect();
        for id in ids {
            hash.insert(id);
        }
        let screen = Box::new(screen);
        std::thread::spawn(move || {
            let mut screen = *screen;
            for msg in rx.iter() {
                match msg.0 {
                    Cmds::Print => screen.print(msg.1, &msg.2).unwrap(),
                    Cmds::Flush => screen.flush(msg.1).unwrap(),
                    Cmds::Println => screen.println(msg.1, &msg.2).unwrap(),
                    Cmds::Break => break,
                };
            }
        });
        Bridge { bridge: tx, hash }
    }
    /// Print a new `line` in window with `id`. Returns `()` if successful, `Err(std::io::ErrorKind)` if not.
    pub fn println(&self, id: usize, msg: &str) -> Result<(), std::io::ErrorKind> {
        self.validate_id(id)?;
        self.bridge
            .send((Cmds::Println, id, msg.to_string()))
            .unwrap();
        Ok(())
    }
    /// Print `line` in window with `id`, but do not flush. Returns `()` if successful, `Err(std::io::ErrorKind)` if not.
    pub fn print(&self, id: usize, msg: &str) -> Result<(), std::io::ErrorKind> {
        self.validate_id(id)?;
        self.bridge
            .send((Cmds::Print, id, msg.to_string()))
            .unwrap();
        Ok(())
    }
    /// Flush window with `id`. Returns `()` if successful, `Err(std::io::ErrorKind)` if not.
    pub fn flush(&self, id: usize) -> Result<(), std::io::ErrorKind> {
        self.validate_id(id)?;
        self.bridge.send((Cmds::Flush, id, "".to_string())).unwrap();
        Ok(())
    }
    fn validate_id(&self, id: usize) -> Result<(), std::io::ErrorKind> {
        if self.hash.contains(&id) {
            return Ok(());
        }
        Err(std::io::ErrorKind::NotFound)
    }
    /// Kills `Bridge`'s communication, and terminates the `Screen` screening process and deletes it.
    pub fn kill(&self) {
        self.bridge.send((Cmds::Break, 0, "".to_string())).unwrap();
    }
}

impl std::clone::Clone for Bridge {
    fn clone(&self) -> Self {
        let bridge = self.bridge.clone();
        let hash = self.hash.clone();
        Bridge { bridge, hash }
    }
    fn clone_from(&mut self, source: &Self) {
        *self = Bridge::clone(source);
    }
}
