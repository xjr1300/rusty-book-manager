use kernel::model::book::Book;
use kernel::model::id::BookId;

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        Self {
            id: value.book_id,
            title: value.title,
            author: value.author,
            isbn: value.isbn,
            description: value.description,
        }
    }
}
