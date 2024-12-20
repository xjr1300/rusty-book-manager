use std::sync::Arc;

use adapter::database::ConnectionPool;
use adapter::redis::RedisClient;
use adapter::repository::auth::AuthRepositoryImpl;
use adapter::repository::book::BookRepositoryImpl;
use adapter::repository::checkout::CheckoutRepositoryImpl;
use adapter::repository::health::HealthCheckRepositoryImpl;
use adapter::repository::user::UserRepositoryImpl;
use kernel::repository::auth::AuthRepository;
use kernel::repository::book::BookRepository;
use kernel::repository::checkout::CheckoutRepository;
use kernel::repository::health::HealthCheckRepository;
use kernel::repository::user::UserRepository;
use shared::config::AppConfig;

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;

#[mockall::automock]
pub trait AppRegistryExt {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository>;
    fn book_repository(&self) -> Arc<dyn BookRepository>;
    fn auth_repository(&self) -> Arc<dyn AuthRepository>;
    fn checkout_repository(&self) -> Arc<dyn CheckoutRepository>;
    fn user_repository(&self) -> Arc<dyn UserRepository>;
}

/// DIコンテナ
#[derive(Clone)]
pub struct AppRegistryImpl {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
    checkout_repository: Arc<dyn CheckoutRepository>,
}

impl AppRegistryImpl {
    pub fn new(
        pool: ConnectionPool,
        redis_client: Arc<RedisClient>,
        app_config: AppConfig,
    ) -> Self {
        let health_check_repository = HealthCheckRepositoryImpl::new(pool.clone());
        let book_repository = BookRepositoryImpl::new(pool.clone());
        let auth_repository =
            AuthRepositoryImpl::new(pool.clone(), Arc::clone(&redis_client), app_config.auth.ttl);
        let user_repository = UserRepositoryImpl::new(pool.clone());
        let checkout_repository = CheckoutRepositoryImpl::new(pool.clone());
        Self {
            health_check_repository: Arc::new(health_check_repository),
            book_repository: Arc::new(book_repository),
            auth_repository: Arc::new(auth_repository),
            user_repository: Arc::new(user_repository),
            checkout_repository: Arc::new(checkout_repository),
        }
    }
}

impl AppRegistryExt for AppRegistryImpl {
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        Arc::clone(&self.health_check_repository)
    }

    fn book_repository(&self) -> Arc<dyn BookRepository> {
        Arc::clone(&self.book_repository)
    }

    fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        Arc::clone(&self.auth_repository)
    }

    fn checkout_repository(&self) -> Arc<dyn CheckoutRepository> {
        Arc::clone(&self.checkout_repository)
    }

    fn user_repository(&self) -> Arc<dyn UserRepository> {
        Arc::clone(&self.user_repository)
    }
}
