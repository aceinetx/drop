use std::ops::Deref;

pub enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    pub fn is_nothing(&self) -> bool {
        matches!(*self, Maybe::Nothing)
    }

    pub fn is_just(&self) -> bool {
        !self.is_nothing()
    }

    pub const fn as_ref(&self) -> Maybe<&T> {
        match *self {
            Maybe::Just(ref x) => Maybe::Just(x),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Maybe::Nothing => panic!("unwrapping an empty Maybe"),
            Maybe::Just(val) => val,
        }
    }
}

impl<T> Deref for Maybe<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref().unwrap()
    }
}
