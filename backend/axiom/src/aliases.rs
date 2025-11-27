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

pub mod result {
    pub type Error = ::anyhow::Error;
    pub type Fallible<T = ()> = ::core::result::Result<T, Error>;

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

    pub trait AnyExt {
        fn ok(self) -> crate::aliases::result::Fallible<Self>
        where
            Self: ::core::marker::Sized;
    }

    impl<T: ::core::marker::Sized> AnyExt for T {
        fn ok(self) -> crate::aliases::result::Fallible<Self> {
            crate::aliases::result::Fallible::Ok(self)
        }
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
