use crate::memory::InnerData;

#[derive(Debug)]
pub struct Stack {
    pub data: Vec<InnerData>,
    pub head: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            head: 0,
        }
    }

    pub fn push(&mut self, value: InnerData) {
        self.data.push(value);
        self.head += 1;
    }

    pub fn pop(&mut self) -> Option<InnerData> {
        match self.data.pop() {
            Some(value) => {
                self.head -= 1;
                Some(value)
            },
            None => None,
        }
    }

    pub fn data(&self) -> &[InnerData] {
        &self.data
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn top(&self) -> &InnerData {
        self.data.get(self.head - 1).unwrap()
    }
}