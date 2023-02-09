mod buffer;
use buffer::Buffer;

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

#[derive(Debug)]
pub enum Priority {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Window {
    id: usize,
    pub name: String,
    pub buffer: Buffer,
    pub left_child: Option<usize>,
    pub down_child: Option<usize>,
    pub priority: Option<Priority>,
}

impl Window {
    pub fn new(id: usize) -> Window {
        Window {
            id,
            name: format!("Window {}", id),
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
