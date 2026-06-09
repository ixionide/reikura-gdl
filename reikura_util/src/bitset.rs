pub struct BitSet<B: AsRef<[u8]> + AsMut<[u8]> = Vec<u8>> {
    inner: B,
    count: usize,
}

impl BitSet<Vec<u8>> {
    pub fn new(bit_count: usize) -> Self {
        // let byte_count = (count - 1) / 8 + 1;
        // let byte_count = (count + 7) / 8;
        let byte_count = bit_count.div_ceil(8);

        Self {
            inner: vec![0; byte_count],
            count: bit_count,
        }
    }

    pub fn resize(&mut self, count: usize) {
        if count < self.count {
            return;
        }

        let byte_count = count.div_ceil(8);
        self.count = count;

        if byte_count > self.inner.len() {
            self.inner.resize(byte_count, 0);
        }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> BitSet<B> {
    pub fn from_raw(bytes: B, bit_count: usize) -> Self {
        let byte_count = bit_count.div_ceil(8);

        debug_assert!(byte_count == bytes.as_ref().len());

        Self {
            inner: bytes,
            count: bit_count,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.count {
            return None;
        }

        let byte_index = index / 8;
        let bitmask = 1 << (index & 7);
        let byte = self.inner.as_ref().get(byte_index)?;

        Some(byte & bitmask != 0)
    }

    pub fn set(&mut self, index: usize, value: bool) -> Option<bool> {
        if index >= self.count {
            return None;
        }

        let byte_index = index / 8;
        let bitmask = 1 << (index & 7);
        let byte = self.inner.as_mut().get_mut(byte_index)?;
        match value {
            true => *byte |= bitmask,
            false => *byte &= !bitmask,
        }

        Some(value)
    }

    pub fn toggle(&mut self, index: usize) -> Option<bool> {
        if index >= self.count {
            return None;
        }

        let byte_index = index / 8;
        let bitmask = 1 << (index & 7);
        let byte = self.inner.as_mut().get_mut(byte_index)?;
        *byte ^= bitmask;

        Some((*byte & bitmask) != 0)
    }
}
