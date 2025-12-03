use ::axiom::prelude::*;

#[async_trait]
pub trait EventRepository {
    async fn save(self: ::std::sync::Arc<Self>, event: ::domain::Event) -> ::axiom::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::Event>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        ::axiom::result::Fallible::Ok(self.get_by_id(id).await?.is_some())
    }

    async fn search(
        self: ::std::sync::Arc<Self>, filter: EventRepositorySearchFilter,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>>;

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>> {
        self.search(::core::default::Default::default()).await?.into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::default::Default, ::bon::Builder)]
#[builder(on(_, into))]
pub struct EventRepositorySearchFilter {
    pub query: ::core::option::Option<::axiom::string::String>,

    pub statuses: ::core::option::Option<::std::vec::Vec<EventRepositorySearchFilterEventStatus>>,
    pub timestamps: ::core::ops::Range<::core::option::Option<::axiom::time::Timestamp>>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum EventRepositorySearchFilterEventStatus {
    Created,
    Updated,
    Approved,
    Rejected,
}

impl ::core::convert::From<::domain::EventStatus> for EventRepositorySearchFilterEventStatus {
    fn from(value: ::domain::EventStatus) -> Self {
        match value {
            ::domain::EventStatus::Created { .. } => Self::Created,
            ::domain::EventStatus::Updated { .. } => Self::Updated,
            ::domain::EventStatus::Approved { .. } => Self::Approved,
            ::domain::EventStatus::Rejected { .. } => Self::Rejected,
        }
    }
}

#[async_trait]
pub trait EventRecommender {
    async fn track_approved(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible;
    async fn untrack_approved(self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid) -> ::axiom::result::Fallible;

    async fn track_posted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;
    async fn untrack_posted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;

    async fn track_subscribed(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;
    async fn track_reacted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;
    async fn track_commented(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;
    async fn untrack_subscribed(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;
    async fn untrack_reacted(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;
    async fn untrack_commented(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible;

    async fn view_recently_approved(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>>;

    async fn view_recently_posted(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>>;

    async fn view_trending(self: ::std::sync::Arc<Self>)
        -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>>;

    async fn view_personalized(
        self: ::std::sync::Arc<Self>, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::Event>>;
}

#[async_trait]
pub trait EventExporter {
    async fn export_as_csv(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::boxed::Box<[u8]>>;
    async fn export_as_json(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::boxed::Box<[u8]>>;
}

#[async_trait]
pub trait EventRegistrationRepository {
    async fn save(self: ::std::sync::Arc<Self>, registration: ::domain::EventRegistration)
        -> ::axiom::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventRegistration>>;

    async fn get_by_event_and_user_id(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventRegistration>>;

    async fn view_by_event_id(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventRegistration>>;

    async fn view_by_user_id(
        self: ::std::sync::Arc<Self>, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventRegistration>>;
}

#[async_trait]
pub trait EventPostRepository {
    async fn save(self: ::std::sync::Arc<Self>, post: ::domain::EventPost) -> ::axiom::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid) -> ::axiom::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPost>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.get_by_id(post_id).await?.is_some().into_ok()
    }

    async fn view_by_event_id(
        self: ::std::sync::Arc<Self>, event_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPost>>;
}

#[async_trait]
pub trait EventPostReactionRepository {
    async fn save(self: ::std::sync::Arc<Self>, reaction: ::domain::EventPostReaction) -> ::axiom::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, reaction_id: ::domain::Uuid) -> ::axiom::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, reaction_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostReaction>>;

    async fn get_by_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostReaction>>;

    async fn contains_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<bool> {
        self.get_by_post_and_user_id(post_id, user_id).await?.is_some().into_ok()
    }

    async fn view_by_post_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostReaction>>;

    async fn count_by_post_id(self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid) -> ::axiom::result::Fallible<u64> {
        (self.view_by_post_id(post_id).await?.len() as u64).into_ok()
    }
}

#[async_trait]
pub trait EventPostCommentRepository {
    async fn save(self: ::std::sync::Arc<Self>, comment: ::domain::EventPostComment) -> ::axiom::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid) -> ::axiom::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, comment_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::EventPostComment>>;

    async fn view_by_post_and_user_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid, user_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostComment>>;

    async fn view_by_post_id(
        self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::EventPostComment>>;

    async fn count_by_post_id(self: ::std::sync::Arc<Self>, post_id: ::domain::Uuid) -> ::axiom::result::Fallible<u64> {
        (self.view_by_post_id(post_id).await?.len() as u64).into_ok()
    }
}

#[async_trait]
pub trait UserRepository {
    async fn save(self: ::std::sync::Arc<Self>, user: ::domain::User) -> ::axiom::result::Fallible;

    async fn get_by_id(
        self: ::std::sync::Arc<Self>, id: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_email(
        self: ::std::sync::Arc<Self>, email: ::domain::Email,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::User>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::axiom::result::Fallible<bool> {
        self.get_by_id(id).await?.is_some().into_ok()
    }

    async fn contains_username(
        self: ::std::sync::Arc<Self>, username: ::domain::Username,
    ) -> ::axiom::result::Fallible<bool> {
        self.get_by_username(username).await?.is_some().into_ok()
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::domain::Email) -> ::axiom::result::Fallible<bool> {
        self.get_by_email(email).await?.is_some().into_ok()
    }

    async fn search(
        self: ::std::sync::Arc<Self>, filter: UserRepositorySearchFilter,
    ) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::User>>;

    async fn view(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::std::vec::Vec<::domain::User>> {
        self.search(::core::default::Default::default()).await?.into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::default::Default, ::bon::Builder)]
#[builder(on(_, into))]
pub struct UserRepositorySearchFilter {
    pub query: ::core::option::Option<::axiom::string::String>,

    pub roles: ::core::option::Option<::std::vec::Vec<UserRepositoryViewFilterUserRole>>,
    pub statuses: ::core::option::Option<::std::vec::Vec<UserRepositoryViewFilterUserStatus>>,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum UserRepositoryViewFilterUserStatus {
    Created,
    Suspended,
    Unsuspended,
}

impl ::core::convert::From<::domain::UserStatus> for UserRepositoryViewFilterUserStatus {
    fn from(value: ::domain::UserStatus) -> Self {
        match value {
            ::domain::UserStatus::Created => Self::Created,
            ::domain::UserStatus::Suspended { .. } => Self::Suspended,
            ::domain::UserStatus::Unsuspended { .. } => Self::Unsuspended,
        }
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
pub enum UserRepositoryViewFilterUserRole {
    Volunteer,
    EventManager,
    Administrator,
}

impl ::core::convert::From<::domain::UserRole> for UserRepositoryViewFilterUserRole {
    fn from(value: ::domain::UserRole) -> Self {
        match value {
            ::domain::UserRole::Volunteer => Self::Volunteer,
            ::domain::UserRole::EventManager => Self::EventManager,
            ::domain::UserRole::Administrator => Self::Administrator,
        }
    }
}

impl ::core::convert::From<UserRepositoryViewFilterUserRole> for ::domain::UserRole {
    fn from(value: UserRepositoryViewFilterUserRole) -> Self {
        match value {
            UserRepositoryViewFilterUserRole::Volunteer => ::domain::UserRole::Volunteer,
            UserRepositoryViewFilterUserRole::EventManager => ::domain::UserRole::EventManager,
            UserRepositoryViewFilterUserRole::Administrator => ::domain::UserRole::Administrator,
        }
    }
}

#[async_trait]
pub trait UserExporter {
    async fn export_volunteers_as_csv(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::std::boxed::Box<[u8]>>;
    async fn export_volunteers_as_json(
        self: ::std::sync::Arc<Self>,
    ) -> ::axiom::result::Fallible<::std::boxed::Box<[u8]>>;
}

#[async_trait]
pub trait UuidGenerator {
    async fn generate(self: ::std::sync::Arc<Self>) -> ::axiom::result::Fallible<::domain::Uuid>;

    async fn get_timestamp(
        self: ::std::sync::Arc<Self>, uuid: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::axiom::time::Timestamp>;
}

#[async_trait]
pub trait UuidCodec {
    async fn format(
        self: ::std::sync::Arc<Self>, uuid: ::domain::Uuid,
    ) -> ::axiom::result::Fallible<::axiom::string::String>;
    async fn parse(
        self: ::std::sync::Arc<Self>, uuid: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::domain::Uuid>;
}

#[async_trait]
pub trait TimestampCodec {
    async fn format(
        self: ::std::sync::Arc<Self>, timestamp: ::axiom::time::Timestamp,
    ) -> ::axiom::result::Fallible<::axiom::string::String>;
    async fn parse(
        self: ::std::sync::Arc<Self>, timestamp: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::axiom::time::Timestamp>;
}

#[async_trait]
pub trait AuthenticationTokenGenerator {
    async fn generate(
        self: ::std::sync::Arc<Self>, payload: AuthenticationTokenPayload,
    ) -> ::axiom::result::Fallible<::axiom::string::String>;

    async fn get_payload(
        self: ::std::sync::Arc<Self>, token: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::core::option::Option<AuthenticationTokenPayload>>;

    async fn get_user_id(
        self: ::std::sync::Arc<Self>, token: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::Uuid>> {
        self.get_payload(token).await?.map(|payload| payload.user_id).into_ok()
    }

    async fn get_user_role(
        self: ::std::sync::Arc<Self>, token: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::core::option::Option<::domain::UserRole>> {
        self.get_payload(token).await?.map(|payload| payload.user_role).into_ok()
    }

    async fn get_expiry_timestamp(
        self: ::std::sync::Arc<Self>, token: ::axiom::string::String,
    ) -> ::axiom::result::Fallible<::core::option::Option<::axiom::time::Timestamp>> {
        self.get_payload(token).await?.map(|payload| payload.expiry_timestamp).into_ok()
    }

    async fn verify(self: ::std::sync::Arc<Self>, token: ::axiom::string::String) -> ::axiom::result::Fallible<bool> {
        self.get_payload(token).await?.is_some().into_ok()
    }
}

#[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::bon::Builder)]
#[builder(on(_, into))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        from = "AuthenticationTokenPayloadSerdeImpl",
        into = "AuthenticationTokenPayloadSerdeImpl",
        rename_all = "camelCase"
    )
)]
pub struct AuthenticationTokenPayload {
    pub user_id: ::domain::Uuid,
    pub user_role: ::domain::UserRole,
    pub expiry_timestamp: ::axiom::time::Timestamp,
}

#[cfg(feature = "serde")]
#[derive(::serde::Serialize, ::serde::Deserialize, ::bon::Builder)]
#[builder(on(_, into))]
struct AuthenticationTokenPayloadSerdeImpl {
    user_id: AuthenticationTokenPayloadSerdeImplUuid,
    user_role: AuthenticationTokenPayloadSerdeImplUntaggedUserRole,
    expiry_timestamp: ::axiom::time::Timestamp,
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
    ) -> ::axiom::result::Fallible<::domain::PasswordDigest>;

    async fn verify(
        self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest,
    ) -> ::axiom::result::Fallible<bool> {
        (self.hash(password).await? == digest).into_ok()
    }
}

#[async_trait]
pub trait Notifier {}
