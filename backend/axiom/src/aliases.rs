pub mod option {
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
}

pub mod result {
    pub type Error = ::anyhow::Error;
    pub type Fallible<T = ()> = ::core::result::Result<T, Error>;

    /// Assumes: **(1)** `$ok` is of type `<$ident>OkResponse`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! ok {
        ($ident:tt | $ok:expr) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Ok($ok))
            }
        };

        ($ident:tt) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Ok(()))
            }
        };
    }

    /// Assumes: **(1)** `$errs` is of type `::std::vec::Vec<<$ident>ErrResponse>`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! errs {
        ($ident:tt | $errs:expr) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Err($errs))
            }
        };
    }

    /// Assumes: **(1)** `$err` is of type `<$ident>ErrResponse`; **(2)** `paste` is within scope.
    #[macro_export]
    macro_rules! err {
        ($ident:tt | $err:expr) => {
            ::paste::paste! {
                ::axiom::aliases::result::Fallible::Ok([<$ident Response>]::Err(::std::vec![[<$ident ErrResponse>]::$err]))
            }
        };
    }

    pub use errs;
    pub use err;
}

/*
return ::axiom::result::Fallible::Ok(ViewPublishedEventsResponse::Err(::std::vec![ViewPublishedEventsErrResponse::UserNotFound]));
*/

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
