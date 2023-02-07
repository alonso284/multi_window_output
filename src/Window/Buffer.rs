const BUFFER_SIZE: usize = 64;
const INIT: Option<String> = None;

#[cfg(test)]
mod buffer_tests {
    use super::*;
    #[test]
    fn print_buffer() {
        let mut buffer = Buffer::new();

        for _ in 0..1026 {
            buffer.append(&format!("Hello World"));
            buffer.flush();
        }

        for i in buffer.iter() {
            assert_eq!(i, &Some(String::from("Hello World")));
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    queue: [Option<String>; BUFFER_SIZE],
    start_pointer: usize,
    end_pointer: usize,
}

pub struct BufferIterator<'a> {
    queue_pointer: &'a [Option<String>],
    start_pointer: usize,
    end_pointer: usize,
}

impl Buffer {
    // Returna new empty buffer
    pub fn new() -> Buffer {
        Buffer {
            queue: [INIT; BUFFER_SIZE],
            start_pointer: 0,
            end_pointer: 0,
        }
    }
    // Append to last written line
    pub fn append(&mut self, suffix: &str) {
        match &mut self.queue[self.end_pointer] {
            Some(line) => line.push_str(suffix),
            None => self.queue[self.end_pointer] = Some(suffix.to_string()),
        }
    }
    // Go to next line
    pub fn flush(&mut self) {
        self.end_pointer = (self.end_pointer + 1) % BUFFER_SIZE;
        if self.start_pointer == self.end_pointer {
            self.queue[self.end_pointer] = None;
            self.start_pointer = (self.start_pointer + 1) % BUFFER_SIZE;
        }
    }
    // Return buffer iterator
    pub fn iter(&self) -> BufferIterator {
        BufferIterator {
            queue_pointer: &self.queue,
            start_pointer: self.start_pointer,
            end_pointer: self.end_pointer,
        }
    }
}

impl<'a> Iterator for BufferIterator<'a> {
    type Item = &'a Option<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_pointer != self.end_pointer {
            let borrow = &self.queue_pointer[self.start_pointer];
            self.start_pointer = (self.start_pointer + 1) % BUFFER_SIZE;
            Some(borrow)
        } else {
            None
        }
    }
}
