use memmap2::MmapMut;

pub struct Variables<V = Vec<i32>> {
    inner: V,
}

impl<V> Variables<V> {
    pub fn inner(&self) -> &V {
        &self.inner
    }
}

impl Variables<Vec<i32>> {
    pub fn new(var_count: usize) -> Self {
        Self {
            inner: vec![0; var_count],
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn resize(&mut self, var_count: usize) {
        self.inner.resize(var_count, 0);
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        self.inner.get(index).copied()
    }

    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let Some(variable) = self.inner.get_mut(index) else {
            return false;
        };

        *variable = value;

        true
    }

    pub fn inc(&mut self, index: usize) -> bool {
        let Some(variable) = self.inner.get_mut(index) else {
            return false;
        };

        *variable += 1;

        true
    }

    pub fn dec(&mut self, index: usize) -> bool {
        let Some(variable) = self.inner.get_mut(index) else {
            return false;
        };

        *variable -= 1;

        true
    }
}

impl<V> From<V> for Variables<V> {
    fn from(inner: V) -> Self {
        Self { inner }
    }
}

impl Variables<MmapMut> {
    pub fn len(&self) -> usize {
        self.inner.as_ref().len() / size_of::<i32>()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.as_ref().len() == 0
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        let pos = size_of::<i32>() * index;
        let end = pos + size_of::<i32>();

        self.inner
            .get(pos..end)
            .map(|it| i32::from_le_bytes(it.try_into().unwrap()))
    }

    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let pos = size_of::<i32>() * index;
        let end = pos + size_of::<i32>();

        let Some(variable) = self.inner.as_mut().get_mut(pos..end) else {
            return false;
        };

        variable.copy_from_slice(value.to_le_bytes().as_slice());

        true
    }
}
