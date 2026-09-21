use std::{array, mem::MaybeUninit};

type Result<T> = std::result::Result<T, StackError>;

#[derive(Debug, Clone, Copy)]
pub enum StackError {
    Overflow,
    Underflow,
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack overflow"),
            StackError::Underflow => write!(f, "Stack underflow"),
        }
    }
}

impl std::error::Error for StackError {}

pub struct Stack<T, const N: usize> {
    len: usize,
    items: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Stack<T, N> {
    pub fn new() -> Self {
        Self {
            len: 0,
            items: array::from_fn(|_| MaybeUninit::uninit()),
        }
    }

    pub fn cap(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, value: T) -> Result<()> {
        if self.len() >= self.cap() {
            return Err(StackError::Overflow);
        }

        self.items[self.len].write(value);
        self.len += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<T> {
        if self.len == 0 {
            return Err(StackError::Underflow);
        }

        self.len -= 1;

        unsafe { Ok(self.items[self.len].assume_init_read()) }
    }

    pub fn clear(&mut self) {
        let ptr = self.items.as_mut_ptr() as *mut T;
        let len = self.len();

        self.len = 0;

        unsafe {
            let items = std::slice::from_raw_parts_mut(ptr, len);
            std::ptr::drop_in_place(items);
        }
    }
}

impl<T, const N: usize> Drop for Stack<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}
