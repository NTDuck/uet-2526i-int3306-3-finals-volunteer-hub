pub mod option {
    use ::async_trait::async_trait;

    pub trait OptionExt<T> {
        fn some(self) -> crate::aliases::result::Fallible<T>;
    }

    impl<T> OptionExt<T> for ::core::option::Option<T> {
        #[track_caller]
        fn some(self) -> crate::aliases::result::Fallible<T> {
            match self {
                ::core::option::Option::Some(val) => crate::aliases::result::Fallible::Ok(val),
                ::core::option::Option::None => {
                    let location = ::std::panic::Location::caller();
                    crate::aliases::result::Fallible::Err(::anyhow::anyhow!(
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

    #[async_trait]
    pub trait OptionOrElseAsyncExt<T> {
        async fn or_else_async<Fut, F>(self, f: F) -> ::core::option::Option<T>
        where
            F: ::core::ops::FnOnce() -> Fut + ::core::marker::Send,
            Fut: ::core::future::Future<Output = ::core::option::Option<T>> + ::core::marker::Send;
    }

    #[async_trait]
    impl<T> OptionOrElseAsyncExt<T> for ::core::option::Option<T>
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
    }

    #[async_trait]
    pub trait OptionTryOrElseAsyncExt<T, E> {
        async fn try_or_else_async<Fut, F>(self, f: F) -> ::core::result::Result<::core::option::Option<T>, E>
        where
            F: ::core::ops::FnOnce() -> Fut + ::core::marker::Send,
            Fut: ::core::future::Future<Output = Result<Option<T>, E>> + ::core::marker::Send;
    }

    #[async_trait]
    impl<T, E> OptionTryOrElseAsyncExt<T, E> for ::core::option::Option<T>
    where
        T: ::core::marker::Send,
        E: ::core::marker::Send,
    {
        async fn try_or_else_async<Fut, F>(self, f: F) -> ::core::result::Result<::core::option::Option<T>, E>
        where
            F: ::core::ops::FnOnce() -> Fut + ::core::marker::Send,
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
    pub trait IteratorTryCollectAllExt: ::core::iter::Iterator {
        fn try_collect_all<B, C, F, T, E>(self, f: F) -> ::core::result::Result<::std::vec::Vec<T>, ::std::vec::Vec<E>>
        where
            Self: ::core::marker::Sized,
            B: ::core::iter::FromIterator<T>,
            C: ::core::iter::FromIterator<E>,
            F: ::core::ops::Fn(Self::Item) -> ::core::result::Result<T, E>;
    }

    impl<I> IteratorTryCollectAllExt for I
    where
        I: ::core::iter::Iterator,
    {
        fn try_collect_all<B, C, F, T, E>(self, f: F) -> ::core::result::Result<::std::vec::Vec<T>, ::std::vec::Vec<E>>
        where
            Self: ::core::marker::Sized,
            B: ::core::iter::FromIterator<T>,
            C: ::core::iter::FromIterator<E>,
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
        fn into_ok(self) -> crate::aliases::result::Fallible<Self>
        where
            Self: ::core::marker::Sized;
    }

    impl<T: ::core::marker::Sized> IntoFallibleExt for T {
        fn into_ok(self) -> crate::aliases::result::Fallible<Self> {
            crate::aliases::result::Fallible::Ok(self)
        }
    }

    /// Assumes: **(1)** `$ok` is of type `<$ident>OkResponse`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! ok {
        ($ident:ident @ $($ok:tt)*) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Ok($($ok)*))
            }
        };

        ($ident:ident) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Ok(()))
            }
        };
    }

    /// Assumes: **(1)** `$errs` is of type `::std::vec::Vec<<$ident>ErrResponse>`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! errs {
        ($ident:ident @ $($errs:tt)*) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Err($($errs)*))
            }
        };
    }

    /// Assumes: **(1)** `$err` is of type `<$ident>ErrResponse`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! err {
        ($ident:ident @ $($err:tt)*) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Err(::std::vec![[<$ident ErrResponse>]::$($err)*]))
            }
        };
    }
}

pub mod time {
    pub type Timestamp = ::chrono::NaiveDateTime;
    pub type Interval = ::chrono::Duration;

    pub trait TimestampExt {
        fn now() -> Self;
    }

    impl TimestampExt for Timestamp {
        fn now() -> Self {
            ::chrono::Utc::now().naive_utc()
        }
    }
}

pub mod string {
    pub type String = ::std::borrow::Cow<'static, str>;
}
