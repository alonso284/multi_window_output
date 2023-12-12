mod buffer;
use buffer::Buffer;
use crate::colors;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_window() {
        let mut window = Window::new(0);
        window.print(&format!("Hello World"));
        for text in window.buffer.iter() {
            assert_eq!(text, &Some(String::from("Hello World")));
        }
    }
}

// Enum to indicate windows partition
#[derive(Debug)]
pub enum Priority {
    Vertical,
    Horizontal,
}

// Windows object
// Children contain other other window ids
// TODO Use Rc pointer for children
// TODO Set values to private (already accesible through methods)
#[derive(Debug)]
pub struct Window {
    id: usize,
    pub name: String,
    pub color: colors::Color,
    pub buffer: Buffer,
    pub left_child: Option<usize>,
    pub down_child: Option<usize>,
    pub priority: Option<Priority>,
}

// TODO allow user to set Window name
impl Window {
    pub fn new(id: usize) -> Window {
        Window {
            id,
            name: format!("Window {}", id),
            color: colors::Color::Green,
            buffer: Buffer::new(),
            left_child: None,
            down_child: None,
            priority: None,
        }
    }
    pub fn print(&mut self, line: &str) {
        self.buffer.append(line);
    }
    pub fn flush(&mut self) {
        self.buffer.flush();
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
