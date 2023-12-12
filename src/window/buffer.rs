// Default vertical size of Window
// TODO make it dynamic for memory and time efficiency
const BUFFER_SIZE: usize = 64;
// Buffer content default value
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

// Iterator for buffer
// The pointers indicate the is content in range [start, end)
// If iter find the end of the buffer before reaching end, it cycles to the beggining
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
    // TODO handle other non-printable characters
    // TODO Make new line characters input new lines in the buffer (insert in queue)
    fn proccess_str(raw: &str) -> String {
        // Proccessed string
        let mut pro = String::new();
        for letter in raw.chars() {
            match letter {
                // Ignore new line or null characters (it screws with the printing proccess)
                '\n' | '\0' => {},
                // Reduce tab size
                '\t' => { pro.push_str("    "); },
                // Resets the string
                '\r' => { pro = String::new(); },
                letter => { pro.push(letter); },
            };
        }
        pro
    }
    // Append to line at the top or initiate new line if empty
    pub fn append(&mut self, suffix: &str) {
        match &mut self.queue[self.end_pointer] {
            Some(line) => line.push_str(&Self::proccess_str(suffix)),
            None => self.queue[self.end_pointer] = Some(Self::proccess_str(suffix)),
        }
    }
    // Flush content in current line, go to next one
    pub fn flush(&mut self) {
        self.end_pointer = (self.end_pointer + 1) % BUFFER_SIZE;
        // If buffer is full, delete line at front of queue
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

    // Iterator method to cycle through buffer
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
