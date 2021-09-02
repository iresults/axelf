pub enum TriState<T, E> {
    Some(T),
    None,
    Error(E),
}

#[allow(unused)]
impl<T, E> TriState<T, E> {
    pub fn some(v: T) -> Self {
        Self::Some(v)
    }

    pub fn none() -> Self {
        Self::None
    }

    pub fn error(e: E) -> Self {
        Self::Error(e)
    }
}

impl<T, E> From<Result<T, E>> for TriState<T, E> {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(v) => Self::Some(v),
            Err(e) => Self::Error(e),
        }
    }
}
