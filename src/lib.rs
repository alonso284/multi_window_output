mod window;
use window::{Priority, Window};
use termion::screen::IntoAlternateScreen;
use std::io::Write;
use termion::color;
use termion::terminal_size;

const MAX_WIN:usize = 30;
const INIT:Option<Window> = None;
const MAX_WIDTH:usize = 512;
const MAX_HEIGHT:usize = 254;

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

#[derive(Debug)]
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
        let mut scr = std::io::stdout().into_alternate_screen().unwrap();

        let (width, height) = terminal_size().unwrap();
        println!("{}Screen: {}{}{}", color::Bg(color::Green), self.name, " ".repeat((width - 8 - self.name.len() as u16) as usize), color::Bg(color::Reset));
        let width = width as usize;
        let height = height as usize - 1;
        
        let mut buffer = [[(b' ' as u8,false);MAX_WIDTH];MAX_HEIGHT-1];
        self.output(0, 0, width, 0, height, &mut buffer);

        for i in 0..height{
            for j in 0..width{
                if buffer[i as usize][j as usize].1 {
                    print!("{}{}{}", color::Bg(color::Green), buffer[i as usize][j as usize].0 as char, color::Bg(color::Reset));
                } else {
                    print!("{}", buffer[i as usize][j as usize].0 as char);
                }
            }
        }
        scr.flush().unwrap();
    }
    fn output(&self, id:usize, start_width:usize, mut end_width:usize, start_height:usize, mut end_height:usize, buffer: &mut [[(u8, bool);MAX_WIDTH];MAX_HEIGHT-1] ) {
        match self.windows[id].as_ref().unwrap().priority {
            Some(Priority::Vertical) => {
              let mit = (start_width + end_width) / 2;
              let left_id:usize = self.windows[id].as_ref().unwrap().left_child.as_ref().unwrap().clone();
              self.output(left_id, mit, end_width, start_height, end_height, buffer);
              end_width = mit;
              if let Some(down_id) = self.windows[id].as_ref().unwrap().down_child.as_ref() {
                  let mit = (start_height + end_height) / 2;
                  self.output(down_id.clone(), start_width, end_width, mit, end_height, buffer);
                  end_height = mit;
              }
            },
            Some(Priority::Horizontal) => {
              let mit = (start_height + end_height) / 2;
              let down_id:usize = self.windows[id].as_ref().unwrap().down_child.as_ref().unwrap().clone();
              self.output(down_id, start_width, end_width, mit, end_height, buffer);
              end_height = mit;
              if let Some(left_id) = self.windows[id].as_ref().unwrap().left_child.as_ref() {
                  let mit = (start_width + end_width) / 2;
                  self.output(left_id.clone(), mit, end_width, start_height, end_height, buffer);
                  end_width = mit;
              }
            },
            None => {},
        }

        let mut buffer_size = 0;
        for _ in self.windows[id].as_ref().unwrap().buffer.iter() {
            buffer_size += 1;
        }

        let mut it = self.windows[id].as_ref().unwrap().buffer.iter();
        if buffer_size > end_height - start_height - 1 {
            it.nth(buffer_size - (end_height - start_height)).unwrap().as_ref().unwrap();
        }
        
        // Put the content
        let empty_line = format!("-- ");
        for i in start_height..end_height-1 {
            let line = match it.next() { Some(s) => s.as_ref().unwrap(), None => &empty_line };
            // let line = &empty_line;
            let mut letter = line.chars();
            for j in start_width..end_width {
                if j == end_width -1 { buffer[i as usize][j as usize] = (b' ', true )}
                else { buffer[i as usize][j as usize] = ( match letter.next() { Some(l) => l as u8, None => b' ' }, false) };
            }
        }

        // Put the name in the lower part
        let name = format!("ID: {} size {}", id, buffer_size);
        let mut c = name.chars();
        for j in start_width..end_width {
            buffer[end_height - 1][j as usize] = (match c.next() {
               Some(l) => l as u8, 
               None => b' ',
            } , true);
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
            self.windows[id].as_mut().unwrap().priority = Some(priority);
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
