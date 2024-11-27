use serde::{Deserialize, Serialize};

use kernel::model::book::event::CreateBook;
use kernel::model::book::Book;
use kernel::model::id::BookId;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        Self {
            title: value.title,
            author: value.author,
            isbn: value.isbn,
            description: value.description,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        Self {
            id: value.id,
            title: value.title,
            author: value.author,
            isbn: value.isbn,
            description: value.description,
        }
    }
}
