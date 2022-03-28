pub struct Stack {
    pub data: Vec<i8>,
    pub head: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            head: 0,
        }
    }

    pub fn push(&mut self, value: i8) {
        self.data.push(value);
        self.head += 1;
    }

    pub fn pop(&mut self) -> Option<i8> {
        match self.data.pop() {
            Some(value) => {
                self.head -= 1;
                Some(value)
            },
            None => None,
        }
    }

    pub fn data(&self) -> &[i8] {
        &self.data
    }

    pub fn head(&self) -> usize {
        self.head
    }
}