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
        let mut screen = Screen::new("Temp");
        let id_1 = screen.append_left_child(0).unwrap();
        let id_2 = screen.append_down_child(0).unwrap();
        screen.println(id_1, "Hello World").unwrap();
        screen.println(id_2, "Hello World").unwrap();
    }
    #[test]
    fn bridge_creation() {
        let mut screen = Screen::new("Screen using bridge"); 
        let mut bridge = Bridge::new(screen);
        bridge.println(0, "Hello World");
    }
}

#[derive(Debug)]
pub struct Screen {
    name: String,
    windows: [Option<Window>; MAX_WIN],
    count: usize,
    buffer: [[(u8, bool); MAX_WIDTH]; MAX_HEIGHT],
}

impl Screen {
    pub fn new(name_screen: &str, name_window: &str) -> Screen {
        let mut screen = Screen {
            windows: [INIT; MAX_WIN],
            count: 1,
            name: name_screen.to_string(),
            buffer: [[(b' ', false); MAX_WIDTH]; MAX_HEIGHT],
        };
        screen.windows[0] = Some(Window::new(0, name_window));
        screen.load();
        screen
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
                if self.buffer[i as usize][j as usize].1 {
                    print!(
                        "{}{}{}",
                        color::Bg(color::Green),
                        self.buffer[i as usize][j as usize].0 as char,
                        color::Bg(color::Reset)
                    );
                } else {
                    print!("{}", self.buffer[i as usize][j as usize].0 as char);
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
                let left_id: usize = self.windows[id]
                    .as_ref()
                    .unwrap()
                    .left_child
                    .as_ref()
                    .unwrap()
                    .clone();
                self.output(left_id, mit, end_width, start_height, end_height);
                end_width = mit;
                if let Some(down_id) = self.windows[id].as_ref().unwrap().down_child.as_ref() {
                    let mit = (start_height + end_height) / 2;
                    self.output(
                        down_id.clone(),
                        start_width,
                        end_width,
                        mit,
                        end_height,
                    );
                    end_height = mit;
                }
            }
            Some(Priority::Horizontal) => {
                let mit = (start_height + end_height) / 2;
                let down_id: usize = self.windows[id]
                    .as_ref()
                    .unwrap()
                    .down_child
                    .as_ref()
                    .unwrap()
                    .clone();
                self.output(down_id, start_width, end_width, mit, end_height);
                end_height = mit;
                if let Some(left_id) = self.windows[id].as_ref().unwrap().left_child.as_ref() {
                    let mit = (start_width + end_width) / 2;
                    self.output(
                        left_id.clone(),
                        mit,
                        end_width,
                        start_height,
                        end_height,
                    );
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

        let empty_line = format!("-- ");
        for i in start_height..end_height - 1 {
            let line = match it.next() {
                Some(s) => s.as_ref().unwrap(),
                None => &empty_line,
            };
            let mut letter = line.chars();
            for j in start_width..end_width {
                if j == end_width - 1 {
                    self.buffer[i as usize][j as usize] = (b' ', true)
                } else {
                    self.buffer[i as usize][j as usize] = (
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
        let name = format!("{} ID: {}", self.windows[id].as_ref().unwrap().get_name(), id);
        let mut c = name.chars();
        for j in start_width..end_width {
            self.buffer[end_height - 1][j as usize] = (
                match c.next() {
                    Some(l) => l as u8,
                    None => b' ',
                },
                true,
            );
        }
    }
    // Validate existance of window
    fn validate_id(&self, id: usize) -> Result<(), String> {
        // Invalid ID
        if id >= MAX_WIN {
            return Err(format!("ID of window does not exist"));
        }
        // Child does not exist
        if let None = &self.windows[id] {
            return Err(format!("Window does not exist"));
        }
        Ok(())
    }
    pub fn append_left_child(&mut self, id: usize, name: &str) -> Result<usize, String> {
        self.append_child(id, name, Priority::Vertical)
    }
    pub fn append_down_child(&mut self, id: usize, name: &str) -> Result<usize, String> {
        self.append_child(id, name, Priority::Horizontal)
    }
    fn append_child(&mut self, id: usize, name: &str, priority: Priority) -> Result<usize, String> {
        // Not enough space to add new window
        if self.count >= MAX_WIN {
            return Err(format!("Not enough space to add new window"));
        }
        // Validate if child exits
        if let Err(e) = self.validate_id(id) {
            return Err(e);
        }

        // Validate that node doesn't alredy have child
        let child = match priority {
            Priority::Vertical => self.windows[id].as_ref().unwrap().left_child,
            Priority::Horizontal => self.windows[id].as_ref().unwrap().down_child,
        };
        match child {
            Some(id) => return Err(format!("Child already exists at {}", id)),
            None => {}
        }

        self.windows[self.count] = Some(Window::new(self.count, name));
        match priority {
            Priority::Vertical => self.windows[id].as_mut().unwrap().left_child = Some(self.count),
            Priority::Horizontal => {
                self.windows[id].as_mut().unwrap().down_child = Some(self.count)
            }
        };
        if let None = self.windows[id].as_ref().unwrap().priority {
            self.windows[id].as_mut().unwrap().priority = Some(priority);
        }
        self.count += 1;
        Ok(self.count - 1)
    }
    pub fn println(&mut self, id: usize, line: &str) -> Result<(), String> {
        // Validate if child exits
        if let Err(e) = self.validate_id(id) {
            return Err(e);
        }
        self.print(id, line).unwrap();
        self.flush(id)
    }
    pub fn print(&mut self, id: usize, line: &str) -> Result<(), String> {
        // Validate if child exits
        if let Err(e) = self.validate_id(id) {
            return Err(e);
        }
        self.windows[id].as_mut().unwrap().print(line);
        Ok(())
    }
    pub fn flush(&mut self, id: usize) -> Result<(), String> {
        // Validate if child exits
        if let Err(e) = self.validate_id(id) {
            return Err(e);
        }
        self.windows[id].as_mut().unwrap().flush();
        self.load();
        Ok(())
    }
}

enum Cmds {
    Print, Flush, Println, Break,
}

// Bridge is a complement to string so you can send messages to the string from different threads
// You initiate a Bridge by passing a pre-created screen. The only disadvantage of using bridge is
// that you cant create new children once you have created the bridge
pub struct Bridge{
    bridge: std::sync::mpsc::Sender<(Cmds, usize, String)>,
    hash: std::collections::HashSet<usize>,
}

impl Bridge{
    pub fn new(screen: Screen) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<(Cmds, usize, String)>();
        let mut hash:std::collections::HashSet<usize> = std::collections::HashSet::new();
        let ids:Vec<usize> = screen.windows.iter().map(|window| { 
        match &window {
            Some(window) => window.get_id(),
            None => 0,
        }} ).rev().collect();
        for id in ids {
            hash.insert(id);
        }
        let screen = Box::new(screen);
        std::thread::spawn( move || {
            let mut screen = *screen;
            for msg in rx.iter(){
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
    pub fn print(&self, id: usize, msg: &str )  -> Result<(), String> {
        if let Err(e) = self.validate_id(id) {
            return Err(e);
        }
        self.bridge.send((Cmds::Print, id, msg.to_string())).unwrap();
        Ok(())
    }
    pub fn flush(&self, id: usize)  -> Result<(), String> {
        if let Err(e) = self.validate_id(id) {
            return Err(e);
        }
        self.bridge.send((Cmds::Flush, id, "".to_string())).unwrap();
        Ok(())
    }
    pub fn println(&self, id:usize, msg: &str) -> Result<(), String> {
        if let Err(e) = self.validate_id(id) {
            return Err(e);
        }
        self.bridge.send((Cmds::Println, id, msg.to_string())).unwrap();
        Ok(()) 
    }
    fn validate_id(&self, id:usize) -> Result<(), String> {
        if self.hash.contains(&id) {
            return Ok(());
        }
        Err(format!("ID does not exist"))
    }
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
        *self = Bridge::clone(source) ;
    }
}
