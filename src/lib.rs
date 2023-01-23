mod window;
use window::{Priority, Window};
use termion::screen::IntoAlternateScreen;
use termion::color;
use termion::terminal_size;

const MAX_WIN:usize = 30;
const INIT:Option<Window> = None;

#[cfg(test)]
mod screen_tests {
    use super::*;
    #[test]
    fn create_screen(){
        let mut screen = Screen::new(&format!("Temp"));
        let id_1 = screen.append_left_child(0).unwrap();
        let id_2 = screen.append_down_child(0).unwrap();
        screen.println(id_1, &format!("Hello World")).unwrap();
        screen.println(id_2, &format!("Hello World")).unwrap();
    }
}

pub struct Screen{
    name: String,
    windows: [Option<Window>;MAX_WIN],
    count: usize,
}

impl Screen {
    pub fn new(name: &String) -> Screen {
        let mut screen = Screen { windows: [INIT;MAX_WIN], count: 1, name: String::clone(name), };
        screen.windows[0] = Some(Window::new(0));
        screen.println(0, &format!("Succesfully initiated screen"));
        screen
    }
    fn load(&self){
        let mut _scr = std::io::stdout().into_alternate_screen().unwrap();
        let (width, height) = terminal_size().unwrap();
        println!("{}Screen: {}{}{}", color::Bg(color::Green), self.name, " ".repeat((width - 8 - self.name.len() as u16) as usize), color::Bg(color::Reset));
        self.output(0);
    }
    fn output(&self, id:usize) {
        println!("----- Window with {} -----", id);
        for line in self.windows[id].as_ref().unwrap().buffer.iter(){
            println!("{}", line.as_ref().unwrap());
        }
        if let Some(id) = self.windows[id].as_ref().unwrap().left_child {
            self.output(id);
        }
        if let Some(id) = self.windows[id].as_ref().unwrap().down_child {
            self.output(id);
        }
    }
    // Validate existance of window
    fn validate_id(&self, id:usize) -> Result<(), String>{
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
    pub fn append_left_child(&mut self, id:usize) -> Result<usize, String> {
        self.append_child(id, Priority::Vertical)
    }
    pub fn append_down_child(&mut self, id:usize) -> Result<usize, String> {
        self.append_child(id, Priority::Horizontal)
    }
    fn append_child(&mut self, id: usize, priority: Priority) -> Result<usize, String> {
        // Not enough space to add new window
        if self.count >= MAX_WIN {
            return Err(format!("Not enough space to add new window"));
        }
        // Validate if child exits
        if let Err(e) = self.validate_id(id){
            return Err(e);
        }

        // Validate that node doesn't alredy have child
        let child = match priority {
            Priority::Vertical => self.windows[id].as_ref().unwrap().left_child,
            Priority::Horizontal => self.windows[id].as_ref().unwrap().down_child,
        };
        match child {
            Some(id) => return Err(format!("Child already exists at {}", id)),
            None => {},
        }

        self.windows[self.count] = Some(Window::new(self.count));
        match priority {
            Priority::Vertical => self.windows[id].as_mut().unwrap().left_child = Some(self.count),
            Priority::Horizontal => self.windows[id].as_mut().unwrap().down_child = Some(self.count),
        };
        if let None = self.windows[id].as_ref().unwrap().priority{
            self.windows[id].as_mut().unwrap().priority = Some(Priority::Vertical);
        }
        self.count += 1;
        Ok(self.count-1)
    }
    pub fn println(&mut self, id:usize, line: &String) -> Result<(), String> {
        // Validate if child exits
        if let Err(e) = self.validate_id(id){
            return Err(e);
        }
        self.print(id, line).unwrap();
        self.flush(id)
    }
    pub fn print(&mut self, id:usize, line: &String) -> Result<(), String> {
        // Validate if child exits
        if let Err(e) = self.validate_id(id){
            return Err(e);
        }
        self.windows[id].as_mut().unwrap().print(line);
        Ok(())
    }
    pub fn flush(&mut self, id:usize) -> Result<(), String> {
        // Validate if child exits
        if let Err(e) = self.validate_id(id){
            return Err(e);
        }
        self.windows[id].as_mut().unwrap().flush();
        self.load();
        Ok(())
    }
}
