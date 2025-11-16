pub mod option {
    pub trait OptionExt<T> {
        fn ok(self) -> crate::result::Fallible<T>;
    }

    impl<T> OptionExt<T> for ::core::option::Option<T> {
        #[track_caller]
        fn ok(self) -> crate::result::Fallible<T> {
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
}

pub mod result {
    pub type Error = ::anyhow::Error;
    pub type Fallible<T = ()> = ::core::result::Result<T, Error>;
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

// https://docs.rs/once_cell/latest/once_cell/#lazily-compiled-regex
#[macro_export]
macro_rules! regex {
    ($regex:literal $(,)?) => {{
        static REGEX: ::once_cell::sync::OnceCell<::regex::Regex> = ::once_cell::sync::OnceCell::new();
        REGEX.get_or_init(|| ::regex::Regex::new($regex).unwrap())
    }};
}
