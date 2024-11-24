use std::sync::Arc;

use adapter::database::ConnectionPool;
use adapter::repository::health::HealthCheckRepositoryImpl;
use kernel::repository::health::HealthCheckRepository;

/// DIコンテナ
#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repository = HealthCheckRepositoryImpl::new(pool.clone());
        Self {
            health_check_repository: Arc::new(health_check_repository),
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        Arc::clone(&self.health_check_repository)
    }
}
