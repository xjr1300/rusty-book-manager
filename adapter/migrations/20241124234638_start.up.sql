-- updated_at列を自動更新する関数
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := now();
    return NEW;
END;
$$ LANGUAGE plpgsql;

-- booksテーブルの作成
CREATE TABLE IF NOT EXISTS books(
    book_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    isbn VARCHAR(255) NOT NULL,
    description VARCHAR(1024) NOT NULL,
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

-- booksテーブルのupdated_at列を自動更新するトリガーを登録
CREATE TRIGGER books_updated_at_trigger
    BEFORE UPDATE ON books FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();
