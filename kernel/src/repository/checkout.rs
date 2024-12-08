use async_trait::async_trait;

use shared::error::AppResult;

use crate::model::checkout::event::{CreateCheckout, UpdateReturned};
use crate::model::checkout::Checkout;
use crate::model::id::{BookId, UserId};

#[async_trait]
#[mockall::automock]
pub trait CheckoutRepository: Send + Sync {
    /// 蔵書を貸出する。
    async fn create(&self, event: CreateCheckout) -> AppResult<()>;
    /// 蔵書を返却する。
    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()>;
    /// すべての未返却の貸出を返す。
    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>>;
    /// ユーザーの未返却の貸出を返す。
    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>>;
    /// 蔵書の返却済みを含む貸出履歴を返す。
    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>>;
}
