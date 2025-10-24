use ::async_trait::async_trait;

#[async_trait]
pub trait EventRepository {

}

#[async_trait]
pub trait VolunteerRepository {
    // something to find option<id> based on username or email
    
    async fn get_by_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<::core::option::Option<::domain::Volunteer>>;
    async fn get_by_username(self: ::std::sync::Arc<Self>, username: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::Volunteer>>;
    async fn get_by_email(self: ::std::sync::Arc<Self>, email: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::Volunteer>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_id(id).await?.is_some())
    }

    async fn contains_username(self: ::std::sync::Arc<Self>, username: ::aliases::string::String) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_username(username).await?.is_some())
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::aliases::string::String) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_email(email).await?.is_some())
    }
}

#[async_trait]
pub trait EventManagerRepository {
    async fn get_by_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<::core::option::Option<::domain::EventManager>>;
    async fn get_by_username(self: ::std::sync::Arc<Self>, username: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::EventManager>>;
    async fn get_by_email(self: ::std::sync::Arc<Self>, email: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::EventManager>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_id(id).await?.is_some())
    }

    async fn contains_username(self: ::std::sync::Arc<Self>, username: ::aliases::string::String) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_username(username).await?.is_some())
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::aliases::string::String) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_email(email).await?.is_some())
    }
}

#[async_trait]
pub trait AdministratorRepository {
    async fn get_by_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<::core::option::Option<::domain::Administrator>>;
    async fn get_by_username(self: ::std::sync::Arc<Self>, username: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::Administrator>>;
    async fn get_by_email(self: ::std::sync::Arc<Self>, email: ::aliases::string::String) -> ::aliases::result::Fallible<::core::option::Option<::domain::Administrator>>;

    async fn contains_id(self: ::std::sync::Arc<Self>, id: ::domain::Uuid) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_id(id).await?.is_some())
    }

    async fn contains_username(self: ::std::sync::Arc<Self>, username: ::aliases::string::String) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_username(username).await?.is_some())
    }

    async fn contains_email(self: ::std::sync::Arc<Self>, email: ::aliases::string::String) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.get_by_email(email).await?.is_some())
    }
}

#[async_trait]
pub trait PasswordHasher {
    async fn hash(self: ::std::sync::Arc<Self>, password: ::domain::Password) -> ::aliases::result::Fallible<::domain::PasswordDigest>;

    async fn verify(self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.hash(password).await? == digest)
    }
}
