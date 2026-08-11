pub enum LazyResult<T, E> {
    Uninit(Box<dyn FnOnce() -> Result<T, E>>),
    Ok(T),
    Err(E),
}

impl<T, E> LazyResult<T, E> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<T, E> + 'static,
    {
        Self::Uninit(Box::new(f))
    }

    pub fn get(&mut self) -> Result<&T, &E> {
        let f = match self {
            Self::Uninit(f) => std::mem::replace(f, Box::new(|| unreachable!())),
            Self::Ok(ok) => return Ok(ok),
            Self::Err(err) => return Err(err),
        };

        *self = match f() {
            Ok(ok) => Self::Ok(ok),
            Err(err) => Self::Err(err),
        };

        self.get()
    }
}
