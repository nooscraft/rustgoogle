#![allow(missing_docs)]

pub fn by_box(b: &Box<u32>) -> u32 {
    **b
}

pub fn by_vec(v: &Vec<u32>) -> usize {
    v.len()
}

pub struct Bag(Vec<u32>);

impl Bag {
    pub fn new() -> Self {
        Bag(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn add(self, other: Bag) -> Bag {
        let mut v = self.0;
        v.extend(other.0);
        Bag(v)
    }

    pub fn into_vec(&self) -> Vec<u32> {
        self.0.clone()
    }
}
