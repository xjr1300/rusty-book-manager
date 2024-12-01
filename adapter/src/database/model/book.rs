use kernel::model::book::Book;
use kernel::model::id::{BookId, UserId};
use kernel::model::user::BookOwner;

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owned_by: UserId,
    pub owner_name: String,
}

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        Self {
            id: value.book_id,
            title: value.title,
            author: value.author,
            isbn: value.isbn,
            description: value.description,
            owner: BookOwner {
                id: value.owned_by,
                name: value.owner_name,
            },
        }
    }
}

pub struct PaginatedBookRow {
    pub total: i64,
    pub id: BookId,
}
