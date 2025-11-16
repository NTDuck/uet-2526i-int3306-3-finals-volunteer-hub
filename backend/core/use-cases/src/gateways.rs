use ::async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn view_recently_approved(self: ::std::sync::Arc<Self>, limit: usize) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;
    async fn view_recently_posted(self: ::std::sync::Arc<Self>, limit: usize) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;
    async fn view_trending(self: ::std::sync::Arc<Self>, limit: usize) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;

    async fn view(self: ::std::sync::Arc<Self>, filter: EventRepositoryViewFilter) -> ::aliases::result::Fallible<::std::vec::Vec<::domain::Event>>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::bon::Builder)]
#[builder(on(_, into))]
pub struct EventRepositoryViewFilter {
    #[builder(with = |value: ::std::vec::Vec<impl ::core::convert::Into<EventRepositoryViewFilterEventStatus>>| value.into_iter().map(::core::convert::Into::into).collect())]
    pub statuses: ::std::vec::Vec<EventRepositoryViewFilterEventStatus>,

    pub name: ::core::option::Option<::aliases::string::String>,
    pub description: ::core::option::Option<::aliases::string::String>,
    pub category: ::core::option::Option<::aliases::string::String>,
    pub location: ::core::option::Option<::aliases::string::String>,

    pub timestamps: ::core::ops::Range<::core::option::Option<::aliases::time::Timestamp>>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum EventRepositoryViewFilterEventStatus {
    Created,
    Approved,
    Rejected,
    Completed,
}

impl ::core::convert::From<::domain::EventStatus> for EventRepositoryViewFilterEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
            ::domain::EventStatus::Completed { .. } => Self::Completed,
        }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn save(self: ::std::sync::Arc<Self>, user: ::domain::User) -> ::aliases::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_email(
        self: ::std::sync::Arc<Self>, email: ::domain::Email,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_id(id).await?.is_some())
    }

    async fn contains_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_username(username).await?.is_some())
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::domain::Email) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_email(email).await?.is_some())
    }
}

#[async_trait]
pub trait UuidGenerator {
    async fn generate(self: ::std::sync::Arc<Self>) -> ::aliases::result::Fallible<::domain::Uuid>;

    async fn get_timestamp(
        self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid,
    ) -> ::aliases::result::Fallible<::aliases::time::Timestamp>;
}

#[async_trait]
pub trait UuidCodec {
    async fn format(self: ::std::sync::Arc<Self>, uuid: ::domain::Uuid) -> ::aliases::result::Fallible<::aliases::string::String>;
    async fn parse(self: ::std::sync::Arc<Self>, uuid: ::aliases::string::String) -> ::aliases::result::Fallible<::domain::Uuid>;
}

#[async_trait]
pub trait AuthenticationTokenGenerator {
    async fn generate(
        self: ::std::sync::Arc<Self>, payload: AuthenticationTokenPayload,
    ) -> ::aliases::result::Fallible<::aliases::string::String>;

    async fn get_payload(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<AuthenticationTokenPayload>>;

    async fn get_user_id(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::Uuid>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|payload| payload.user_id))
    }

    async fn get_user_role(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<::domain::UserRole>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|payload| payload.user_role))
    }

    async fn get_expiry_timestamp(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<::core::option::Option<::aliases::time::Timestamp>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|payload| payload.expiry_timestamp))
    }

    async fn verify(
        self: ::std::sync::Arc<Self>, token: ::aliases::string::String,
    ) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.is_some())
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "AuthenticationTokenPayloadSerdeImpl", into = "AuthenticationTokenPayloadSerdeImpl", rename_all = "camelCase")
)]
pub struct AuthenticationTokenPayload {
    pub user_id: ::domain::Uuid,
    pub user_role: ::domain::UserRole,
    pub expiry_timestamp: ::aliases::time::Timestamp,
}

#[cfg(feature = "serde")]
#[derive(::serde::Serialize, ::serde::Deserialize, ::bon::Builder)]
#[builder(on(_, into))]
struct AuthenticationTokenPayloadSerdeImpl {
    user_id: AuthenticationTokenPayloadSerdeImplUuid,
    user_role: AuthenticationTokenPayloadSerdeImplUntaggedUserRole,
    expiry_timestamp: ::aliases::time::Timestamp,
}

#[cfg(feature = "serde")]
impl ::core::convert::From<AuthenticationTokenPayloadSerdeImpl> for AuthenticationTokenPayload {
    fn from(value: AuthenticationTokenPayloadSerdeImpl) -> Self {
        Self::builder()
            .user_id(value.user_id)
            .user_role(value.user_role)
            .expiry_timestamp(value.expiry_timestamp)
            .build()
    }
}

#[cfg(feature = "serde")]
impl ::core::convert::From<AuthenticationTokenPayload> for AuthenticationTokenPayloadSerdeImpl {
    fn from(value: AuthenticationTokenPayload) -> Self {
        Self::builder()
            .user_id(value.user_id)
            .user_role(value.user_role)
            .expiry_timestamp(value.expiry_timestamp)
            .build()
    }
}

#[cfg(feature = "serde")]
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(transparent)]
struct AuthenticationTokenPayloadSerdeImplUuid([u8; 16]);

#[::bon::bon]
impl AuthenticationTokenPayloadSerdeImplUuid {
    #[builder]
    pub fn new(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl ::core::ops::Deref for AuthenticationTokenPayloadSerdeImplUuid {
    type Target = [u8; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::core::convert::From<::domain::Uuid> for AuthenticationTokenPayloadSerdeImplUuid {
    fn from(value: ::domain::Uuid) -> Self {
        Self::builder().value(*value).build()
    }
}

impl ::core::convert::From<AuthenticationTokenPayloadSerdeImplUuid> for ::domain::Uuid {
    fn from(value: AuthenticationTokenPayloadSerdeImplUuid) -> Self {
        Self::builder().value(*value).build()
    }
}

#[cfg(feature = "serde")]
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
enum AuthenticationTokenPayloadSerdeImplUntaggedUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for AuthenticationTokenPayloadSerdeImplUntaggedUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<AuthenticationTokenPayloadSerdeImplUntaggedUserRole> for ::domain::UserRole {
    fn from(value: AuthenticationTokenPayloadSerdeImplUntaggedUserRole) -> Self {
        match value {
            AuthenticationTokenPayloadSerdeImplUntaggedUserRole::Volunteer => Self::Volunteer,
            AuthenticationTokenPayloadSerdeImplUntaggedUserRole::EventManager => Self::EventManager,
            AuthenticationTokenPayloadSerdeImplUntaggedUserRole::Administrator => Self::Administrator,
        }
    }
}

#[async_trait]
pub trait PasswordHasher {
    async fn hash(
        self: ::std::sync::Arc<Self>, password: ::domain::Password,
    ) -> ::aliases::result::Fallible<::domain::PasswordDigest>;

    async fn verify(
        self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest,
    ) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.hash(password).await? == digest)
    }
}
