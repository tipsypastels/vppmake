use std::fmt::{self, Display, Write};

#[derive(Debug)]
pub struct Css(String);

impl Css {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn set(mut self, key: impl Display, value: impl Display) -> Self {
        self.set_impl(key, value);
        self
    }

    pub fn setopt(mut self, b: bool, key: impl Display, value: impl Display) -> Self {
        if b {
            self.set_impl(key, value);
        }
        self
    }

    pub fn extend(mut self, pairs: impl IntoIterator<Item = (impl Display, impl Display)>) -> Self {
        for (key, value) in pairs {
            self.set_impl(key, value);
        }
        self
    }

    fn set_impl(&mut self, key: impl Display, value: impl Display) {
        write!(self.0, "{key}:{value};").unwrap();
    }
}

impl Display for Css {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
