#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T: Clone> {
    buffer: Vec<Option<T>>,
    size: usize,
    head: usize,
    tail: usize,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: vec![None; size],
            size: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            let value = self.buffer[self.head].take().unwrap();
            self.head = (self.head + 1) % self.buffer.len();
            self.size -= 1;
            Ok(value)
        }
    }

    pub fn write(&mut self, byte: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            self.buffer[self.tail] = Some(byte);
            self.tail = (self.tail + 1) % self.buffer.len();
            self.size += 1;
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![None; self.buffer.len()];
        self.size = 0;
        self.head = 0;
        self.tail = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.buffer.len()
    }
}
