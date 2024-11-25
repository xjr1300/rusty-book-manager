use async_trait::async_trait;
use uuid::Uuid;

use crate::model::book::event::CreateBook;
use crate::model::book::Book;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(&self, event: CreateBook) -> anyhow::Result<()>;
    async fn find_all(&self) -> anyhow::Result<Vec<Book>>;
    async fn find_by_id(&self, book_id: Uuid) -> anyhow::Result<Option<Book>>;
}
