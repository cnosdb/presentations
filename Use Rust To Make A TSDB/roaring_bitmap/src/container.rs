use crate::Set;

pub(crate) enum Container {
    Array(ArrayContainer),
    Bitmap(BitMapContainer),
}

impl Container {
    pub fn new() -> Self {
        Container::Array(ArrayContainer::new())
    }

    pub fn ensure_correct_container(&mut self) {
        match self {
            Container::Array(c) => {
                if c.array.len() >= 4096 {
                    *self = Container::Bitmap(c.to_bitmap_container());
                }
            }
            Container::Bitmap(c) => {
                if c.len() < 4096 {
                    *self = Container::Array(c.to_array_container());
                }
            }
        }
    }
}

impl Set for Container {
    type Item = u16;

    fn insert(&mut self, val: Self::Item) {
        match self {
            Container::Array(c) => c.insert(val),
            Container::Bitmap(c) => c.insert(val),
        }
        self.ensure_correct_container();
    }

    fn remove(&mut self, val: Self::Item) {
        match self {
            Container::Array(c) => c.remove(val),
            Container::Bitmap(c) => c.remove(val),
        }
        self.ensure_correct_container();
    }

    fn contains(&self, val: Self::Item) -> bool {
        match self {
            Container::Array(c) => c.contains(val),
            Container::Bitmap(c) => c.contains(val),
        }
    }
}

pub struct ArrayContainer {
    array: Vec<u16>,
}

impl ArrayContainer {
    pub fn new() -> Self {
        ArrayContainer { array: vec![] }
    }

    pub fn to_bitmap_container(&self) -> BitMapContainer {
        let mut bitmap_container = BitMapContainer::new();
        for v in self.array.iter().cloned() {
            bitmap_container.insert(v);
        }
        bitmap_container
    }
}

impl Set for ArrayContainer {
    type Item = u16;

    fn insert(&mut self, val: Self::Item) {
        match self.array.binary_search(&val) {
            Ok(_) => (),
            Err(idx) => {
                self.array.insert(idx, val);
            }
        }
    }

    fn remove(&mut self, val: Self::Item) {
        match self.array.binary_search(&val) {
            Ok(idx) => {
                self.array.remove(idx);
            }
            Err(_) => (),
        }
    }

    fn contains(&self, val: Self::Item) -> bool {
        self.array.binary_search(&val).is_ok()
    }
}

pub struct BitMapContainer {
    len: usize,
    bits: Box<[u64; 1024]>,
}

impl BitMapContainer {
    pub fn new() -> Self {
        Self {
            len: 0,
            bits: Box::new([0; 1024]),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn to_array_container(&self) -> ArrayContainer {
        let mut array = Vec::with_capacity(self.len);
        for (index, mut bit) in self.bits.iter().cloned().enumerate() {
            while bit != 0 {
                array.push((u64::trailing_zeros(bit) + (64 * index as u32)) as u16);
                bit &= bit - 1;
            }
        }
        ArrayContainer { array }
    }
}

impl Set for BitMapContainer {
    type Item = u16;

    fn insert(&mut self, val: Self::Item) {
        let idx = val as usize / 64;
        let offset = val as u64 % 64;
        self.bits[idx] |= 1 << offset;
        self.len += 1;
    }

    fn remove(&mut self, val: Self::Item) {
        let idx = val as usize / 64;
        let offset = val as u64 % 64;
        self.bits[idx] &= !(1 << offset);
        self.len -= 1;
    }

    fn contains(&self, val: Self::Item) -> bool {
        let idx = val as usize / 64;
        let offset = val as u64 % 64;
        (self.bits[idx] & (1 << offset)) != 0
    }
}
