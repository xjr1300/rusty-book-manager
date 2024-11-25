use std::sync::Arc;

use adapter::database::ConnectionPool;
use adapter::repository::book::BookRepositoryImpl;
use adapter::repository::health::HealthCheckRepositoryImpl;
use kernel::repository::book::BookRepository;
use kernel::repository::health::HealthCheckRepository;

/// DIコンテナ
#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repository = HealthCheckRepositoryImpl::new(pool.clone());
        let book_repository = BookRepositoryImpl::new(pool.clone());
        Self {
            health_check_repository: Arc::new(health_check_repository),
            book_repository: Arc::new(book_repository),
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        Arc::clone(&self.health_check_repository)
    }

    pub fn book_repository(&self) -> Arc<dyn BookRepository> {
        Arc::clone(&self.book_repository)
    }
}
