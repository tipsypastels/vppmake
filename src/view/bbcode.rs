use anyhow::Result;
use std::fmt::{self, Display, Write};

#[derive(Debug)]
pub struct BBCode(String);

impl BBCode {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn tag<T>(&mut self, name: impl Display, f: impl FnOnce(&mut Self) -> T) -> T::WithThis<'_>
    where
        T: BBCodeCallbackOutput,
    {
        self.tag_impl(name, None::<std::convert::Infallible>, f)
    }

    pub fn tag_with<T>(
        &mut self,
        name: impl Display,
        value: impl Display,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T::WithThis<'_>
    where
        T: BBCodeCallbackOutput,
    {
        self.tag_impl(name, Some(value), f)
    }

    pub fn text(&mut self, text: impl Display) {
        write!(self.0, "{text}").unwrap();
    }

    fn tag_impl<T>(
        &mut self,
        name: impl Display,
        value: Option<impl Display>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T::WithThis<'_>
    where
        T: BBCodeCallbackOutput,
    {
        if let Some(value) = value {
            write!(self.0, "[{name}=\"{value}\"]").unwrap();
        } else {
            write!(self.0, "[{name}]").unwrap();
        }

        let ret = f(self);
        write!(self.0, "[/{name}]").unwrap();
        ret.with_this(self)
    }
}

impl Display for BBCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub trait BBCodeCallbackOutput {
    type WithThis<'a>;
    fn with_this<'a>(self, this: &'a mut BBCode) -> Self::WithThis<'a>;
}

impl BBCodeCallbackOutput for () {
    type WithThis<'a> = &'a mut BBCode;
    fn with_this<'a>(self, this: &'a mut BBCode) -> Self::WithThis<'a> {
        this
    }
}

impl BBCodeCallbackOutput for Result<()> {
    type WithThis<'a> = Result<&'a mut BBCode>;
    fn with_this<'a>(self, this: &'a mut BBCode) -> Self::WithThis<'a> {
        self.map(|()| this)
    }
}
