#[derive(Debug, Clone)]
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FullBuffer,
    EmptyBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer: Vec<Option<T>> = Vec::with_capacity(capacity);
        (0..capacity).for_each(|_| buffer.push(None));
        Self {
            buffer,
            capacity,
            head: 0,
            len: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.head] = Some(element);
        self.head = (self.head + 1) % self.capacity;
        self.len += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        }
        let ind = (self.head + self.capacity - self.len) % self.capacity;
        let element = unsafe { self.buffer[ind].take().unwrap_unchecked() };
        self.len -= 1;
        Ok(element)
    }

    pub fn clear(&mut self) {
        while self.len > 0 {
            let _ = self.read();
        }
    }

    pub fn overwrite(&mut self, element: T) {
        match self.len {
            full if full == self.capacity => {
                let ind = self.head + self.capacity - self.len;
                self.buffer[ind] = Some(element);
                self.head = (self.head + 1) % self.capacity;
            }
            _ => {
                let _ = self.write(element);
            }
        };
    }
}
