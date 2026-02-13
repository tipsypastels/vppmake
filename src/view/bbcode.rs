#![allow(unused)]

use crate::utils::all_the_tuples;
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
        self.tag_impl(name, Some(SingleArg(value)), f)
    }

    pub fn tag_with_unquoted<T>(
        &mut self,
        name: impl Display,
        value: impl Display,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T::WithThis<'_>
    where
        T: BBCodeCallbackOutput,
    {
        self.tag_impl(name, Some(SingleArgUnquoted(value)), f)
    }

    pub fn tag_with_multi<T>(
        &mut self,
        name: impl Display,
        value: impl BBCodeMultiArgs,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T::WithThis<'_>
    where
        T: BBCodeCallbackOutput,
    {
        self.tag_impl(name, Some(value.display()), f)
    }

    pub fn text(&mut self, text: impl Display) -> &mut Self {
        write!(self.0, "{text}").unwrap();
        self
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
            write!(self.0, "[{name}{value}]").unwrap();
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

struct SingleArg<T>(T);

impl<T: Display> Display for SingleArg<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "=\"{}\"", self.0)
    }
}

struct SingleArgUnquoted<T>(T);

impl<T: Display> Display for SingleArgUnquoted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "={}", self.0)
    }
}

trait NamedArg {
    fn display(&self) -> impl Display;
}

impl<K: Display, V: Display> NamedArg for (K, V) {
    fn display(&self) -> impl Display {
        struct Impl<'a, K, V>(&'a K, &'a V);
        impl<K: Display, V: Display> Display for Impl<'_, K, V> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, " {}=\"{}\"", self.0, self.1)
            }
        }
        Impl(&self.0, &self.1)
    }
}

pub trait BBCodeMultiArgs {
    fn display(&self) -> impl Display;
}

macro_rules! tuple_args {
    ($($i:tt $ty:ident),*) => {
        #[allow(unused_parens)]
        impl<$($ty,)*> BBCodeMultiArgs for ($($ty,)*)
        where
            $($ty: NamedArg,)*
        {
            fn display(&self) -> impl Display {
                struct Impl<'a, $($ty,)*>(&'a ($($ty,)*));
                impl<$($ty,)*> fmt::Display for Impl<'_, $($ty,)*>
                where
                    $($ty: NamedArg,)*
                {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        $(write!(f, "{}", self.0.$i.display())?;)*
                        Ok(())
                    }
                }
                Impl(self)

            }
        }
    };
}
all_the_tuples!(tuple_args);

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
