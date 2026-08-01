use memmap2::MmapMut;

pub struct Register<V: AsRef<[i32]> + AsMut<[i32]> = Vec<i32>> {
    inner: V,
}

impl<V: AsRef<[i32]> + AsMut<[i32]>> From<V> for Register<V> {
    fn from(inner: V) -> Self {
        Self { inner }
    }
}

impl Register {
    pub fn new(reg_count: usize) -> Self {
        Self {
            inner: vec![0; reg_count],
        }
    }

    pub fn resize(&mut self, reg_count: usize) {
        self.inner.resize(reg_count, 0);
    }
}

impl<V: AsRef<[i32]> + AsMut<[i32]>> Register<V> {
    pub fn len(&self) -> usize {
        self.inner.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        self.inner.as_ref().get(index).copied()
    }

    pub fn set(&mut self, index: usize, value: i32) -> bool {
        let Some(register) = self.inner.as_mut().get_mut(index) else {
            return false;
        };

        *register = value;

        true
    }

    pub fn inc(&mut self, index: usize) -> bool {
        let Some(register) = self.inner.as_mut().get_mut(index) else {
            return false;
        };

        *register += 1;

        true
    }

    pub fn dec(&mut self, index: usize) -> bool {
        let Some(register) = self.inner.as_mut().get_mut(index) else {
            return false;
        };

        *register -= 1;

        true
    }
}

pub struct MmapReg {
    ptr: *mut i32,
    len: usize,
    _mmap: MmapMut,
}

impl MmapReg {
    pub fn new(mut mmap: MmapMut) -> Self {
        const I32_SIZE: usize = size_of::<i32>();
        const I32_ALIGN: usize = align_of::<i32>();
        let ptr = mmap.as_mut_ptr().cast();

        assert!(mmap.len().is_multiple_of(I32_SIZE));
        assert!((ptr as usize).is_multiple_of(I32_ALIGN));

        let len = mmap.len() / I32_SIZE;

        Self {
            ptr,
            len,
            _mmap: mmap,
        }
    }
}

impl AsRef<[i32]> for MmapReg {
    fn as_ref(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl AsMut<[i32]> for MmapReg {
    fn as_mut(&mut self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
