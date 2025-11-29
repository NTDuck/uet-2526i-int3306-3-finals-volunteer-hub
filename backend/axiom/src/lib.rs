pub use ::axiom_derive::*;

pub mod prelude {
    pub use ::async_trait::async_trait;

    pub use crate::option::OptionExt as _;
    pub use crate::option::IntoOptionExt as _;
    pub use crate::option::OptionAsyncExt as _;

    pub use crate::iter::IteratorExt as _;

    pub use crate::result::IntoFallibleExt as _;
}

pub mod option {
    pub trait OptionExt<T> {
        fn some(self) -> crate::result::Fallible<T>;
    }

    impl<T> OptionExt<T> for ::core::option::Option<T> {
        #[track_caller]
        fn some(self) -> crate::result::Fallible<T> {
            match self {
                ::core::option::Option::Some(val) => crate::result::Fallible::Ok(val),
                ::core::option::Option::None => {
                    let location = ::std::panic::Location::caller();
                    crate::result::Fallible::Err(::anyhow::anyhow!(
                        "called `OptionExt::some()` on a `None` value at {}:{}:{}",
                        location.file(),
                        location.line(),
                        location.column(),
                    ))
                },
            }
        }
    }

    pub trait IntoOptionExt {
        fn into_some(self) -> ::core::option::Option<Self>
        where
            Self: ::core::marker::Sized;
    }

    impl<T: ::core::marker::Sized> IntoOptionExt for T {
        fn into_some(self) -> ::core::option::Option<Self> {
            ::core::option::Option::Some(self)
        }
    }

    #[::async_trait::async_trait]
    pub trait OptionAsyncExt<T> {
        async fn or_else_async<Fut, F>(self, f: F) -> ::core::option::Option<T>
        where
            F: ::core::ops::FnOnce() -> Fut + ::core::marker::Send,
            Fut: ::core::future::Future<Output = ::core::option::Option<T>> + ::core::marker::Send;

        async fn try_or_else_async<Fut, F, E>(self, f: F) -> ::core::result::Result<::core::option::Option<T>, E>
        where
            F: ::core::ops::FnOnce() -> Fut + ::core::marker::Send,
            E: ::core::marker::Send,
            Fut: ::core::future::Future<Output = Result<Option<T>, E>> + ::core::marker::Send;
    }

    #[::async_trait::async_trait]
    impl<T> OptionAsyncExt<T> for ::core::option::Option<T>
    where
        T: ::core::marker::Send,
    {
        async fn or_else_async<Fut, F>(self, f: F) -> ::core::option::Option<T>
        where
            F: ::core::ops::FnOnce() -> Fut + ::core::marker::Send,
            Fut: ::core::future::Future<Output = ::core::option::Option<T>> + ::core::marker::Send,
        {
            match self {
                ::core::option::Option::Some(val) => ::core::option::Option::Some(val),
                ::core::option::Option::None => f().await,
            }
        }

        async fn try_or_else_async<Fut, F, E>(self, f: F) -> ::core::result::Result<::core::option::Option<T>, E>
        where
            F: ::core::ops::FnOnce() -> Fut + ::core::marker::Send,
            E: ::core::marker::Send,
            Fut: ::core::future::Future<Output = ::core::result::Result<::core::option::Option<T>, E>> + ::core::marker::Send,
        {
            match self {
                ::core::option::Option::Some(val) => ::core::result::Result::Ok(::core::option::Option::Some(val)),
                ::core::option::Option::None => f().await,
            }
        }
    }
}

pub mod iter {
    pub trait IteratorExt: ::core::iter::Iterator {
        fn try_collect_all<BT, BE, F, T, E>(self, f: F) -> ::core::result::Result<::std::vec::Vec<T>, ::std::vec::Vec<E>>
        where
            Self: ::core::marker::Sized,
            BT: ::core::iter::FromIterator<T>,
            BE: ::core::iter::FromIterator<E>,
            F: ::core::ops::Fn(Self::Item) -> ::core::result::Result<T, E>;
    }

    impl<I> IteratorExt for I
    where
        I: ::core::iter::Iterator,
    {
        fn try_collect_all<BT, BE, F, T, E>(self, f: F) -> ::core::result::Result<::std::vec::Vec<T>, ::std::vec::Vec<E>>
        where
            Self: ::core::marker::Sized,
            BT: ::core::iter::FromIterator<T>,
            BE: ::core::iter::FromIterator<E>,
            F: ::core::ops::Fn(Self::Item) -> ::core::result::Result<T, E>,
        {
            let (oks, errs) = self
                .map(f)
                .partition::<::std::vec::Vec<_>, _>(::core::result::Result::is_ok);

            if errs.is_empty() {
                let oks = oks.into_iter()
                    .map(|ok| unsafe { ::core::result::Result::unwrap_unchecked(ok) })
                    .collect();

                ::core::result::Result::Ok(oks)

            } else {
                let errs = errs.into_iter()
                    .map(|err| unsafe { ::core::result::Result::unwrap_err_unchecked(err) })
                    .collect();
                
                ::core::result::Result::Err(errs)
            }
        }
    }
}

pub mod result {
    pub type Error = ::anyhow::Error;
    pub type Fallible<T = ()> = ::core::result::Result<T, Error>;

    pub trait IntoFallibleExt {
        fn into_ok(self) -> crate::result::Fallible<Self>
        where
            Self: ::core::marker::Sized;
    }

    impl<T: ::core::marker::Sized> IntoFallibleExt for T {
        fn into_ok(self) -> crate::result::Fallible<Self> {
            crate::result::Fallible::Ok(self)
        }
    }

    /// Assumes: **(1)** `$ok` is of type `<$ident>OkResponse`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! ok {
        ($ident:ident @ $($ok:tt)*) => {
            ::paste::paste! {
                $crate::result::Fallible::Ok([<$ident Response>]::Ok($($ok)*))
            }
        };

        ($ident:ident) => {
            ::paste::paste! {
                $crate::result::Fallible::Ok([<$ident Response>]::Ok(()))
            }
        };
    }

    /// Assumes: **(1)** `$errs` is of type `::std::vec::Vec<<$ident>ErrResponse>`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! errs {
        ($ident:ident @ $($errs:tt)*) => {
            ::paste::paste! {
                $crate::result::Fallible::Ok([<$ident Response>]::Err($($errs)*))
            }
        };
    }

    /// Assumes: **(1)** `$err` is of type `<$ident>ErrResponse`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! err {
        ($ident:ident @ $($err:tt)*) => {
            ::paste::paste! {
                $crate::result::Fallible::Ok([<$ident Response>]::Err(::std::vec![[<$ident ErrResponse>]::$($err)*]))
            }
        };
    }
}

pub mod time {
    pub type Timestamp = ::chrono::DateTime<::chrono::Utc>;
    pub type Interval = ::chrono::Duration;
}

pub mod string {
    pub type String = ::std::borrow::Cow<'static, str>;

    // https://docs.rs/once_cell/latest/once_cell/#lazily-compiled-regex
    /// Assumes: **(1)** `regex` and `once_cell` is within scope.
    #[macro_export]
    macro_rules! regex {
        ($regex:literal $(,)?) => {{
            static REGEX: ::once_cell::sync::OnceCell<::regex::Regex> = ::once_cell::sync::OnceCell::new();
            REGEX.get_or_init(|| ::regex::Regex::new($regex).unwrap())
        }};
    }
}
