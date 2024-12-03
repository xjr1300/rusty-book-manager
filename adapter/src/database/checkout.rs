use sqlx::types::chrono::{DateTime, Utc};

use kernel::model::checkout::{Checkout, CheckoutBook};
use kernel::model::id::{BookId, CheckoutId, UserId};

pub struct CheckoutStateRow {
    pub book_id: BookId,
    pub checkout_id: Option<CheckoutId>,
    pub user_id: Option<UserId>,
}

pub struct CheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<CheckoutRow> for Checkout {
    fn from(value: CheckoutRow) -> Self {
        let book = CheckoutBook {
            book_id: value.book_id,
            title: value.title,
            author: value.author,
            isbn: value.isbn,
        };
        Self {
            id: value.checkout_id,
            book,
            checked_out_by: value.user_id,
            checked_out_at: value.checked_out_at,
            returned_at: None,
        }
    }
}

pub struct ReturnedCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<ReturnedCheckoutRow> for Checkout {
    fn from(value: ReturnedCheckoutRow) -> Self {
        let ReturnedCheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            returned_at,
            title,
            author,
            isbn,
        } = value;
        Self {
            id: checkout_id,
            book: CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
            checked_out_by: user_id,
            checked_out_at,
            returned_at: Some(returned_at),
        }
    }
}
