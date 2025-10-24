use ::async_trait::async_trait;

#[async_trait]
pub trait EventRepository {

}

#[async_trait]
pub trait UserRepository {
    async fn get_by_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_username(self: ::std::sync::Arc<Self>, username: ::domain::Username) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;
    async fn get_by_email(self: ::std::sync::Arc<Self>, email: ::domain::Email) -> ::aliases::result::Fallible<::core::option::Option<::domain::User>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_id(id).await?.is_some())
    }

    async fn contains_username(self: ::std::sync::Arc<Self>, username: ::domain::Username) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_username(username).await?.is_some())
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::domain::Email) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_email(email).await?.is_some())
    }
}

#[async_trait]
pub trait AuthenticationTokenGenerator {
    async fn generate(self: ::std::sync::Arc<Self>, payload: (::domain::Uuid, ::domain::UserRole, ::aliases::time::Timestamp)) -> ::aliases::result::Fallible<::aliases::string::String>;

    async fn get_payload(self: ::std::sync::Arc<Self>, token: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<(::domain::Uuid, ::domain::UserRole, ::aliases::time::Timestamp)>>;

    async fn get_user_id(self: ::std::sync::Arc<Self>, token: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::Uuid>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|(user_id, _, _)| user_id))
    }

    async fn get_user_role(self: ::std::sync::Arc<Self>, token: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::UserRole>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|(_, user_role, _)| user_role))
    }

    async fn get_expiry_timestamp(self: ::std::sync::Arc<Self>, token: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::aliases::time::Timestamp>> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.map(|(_, _, expiry_timestamp)| expiry_timestamp))
    }
    
    async fn verify(self: ::std::sync::Arc<Self>, token: ::aliases::string::String) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_payload(token).await?.is_some())
    }
}

#[async_trait]
pub trait PasswordHasher {
    async fn hash(self: ::std::sync::Arc<Self>, password: ::domain::Password) -> ::aliases::result::Fallible<::domain::PasswordDigest>;

    async fn verify(self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.hash(password).await? == digest)
    }
}
