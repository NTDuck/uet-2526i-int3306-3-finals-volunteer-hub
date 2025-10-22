use ::async_trait::async_trait;

#[async_trait]
pub trait EventRepository {

}

#[async_trait]
pub trait VolunteerRepository {

}

#[async_trait]
pub trait EventManagerRepository {
    
}

#[async_trait]
pub trait AdministratorRepository {
    
}

#[async_trait]
pub trait PasswordHasher {
    async fn hash(self: ::std::sync::Arc<Self>, password: ::domain::Password) -> ::aliases::result::Fallible<::domain::PasswordDigest>;

    async fn verify(self: ::std::sync::Arc<Self>, password: ::domain::Password, digest: ::domain::PasswordDigest) -> ::aliases::result::Fallible<bool> {
        ::aliases::result::Fallible::Ok(self.hash(password).await? == digest)
    }
}
