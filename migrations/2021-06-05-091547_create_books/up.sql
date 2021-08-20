
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    author_id int NOT NULL REFERENCES users (id),
    slug TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    cover TEXT NOT NULL,
    description TEXT NOT NULL,
    body TEXT NOT NULL,
    tags TEXT[] NOT NULL,
    favorites_count int NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX books_author_id_idx ON books (author_id);
-- indices are already created for slugs, as slugs are unique as per the spec

SELECT diesel_manage_updated_at('books');

CREATE TABLE favorite_books (
    user_id int NOT NULL REFERENCES users (id),
    book_id int NOT NULL REFERENCES books (id),
    PRIMARY KEY (user_id, book_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX favorite_books_user_id_idx ON favorite_books (user_id);
CREATE INDEX favorite_books_book_id_idx ON favorite_books (book_id);

SELECT diesel_manage_updated_at('favorite_books');

CREATE TABLE episodes (
    id SERIAL PRIMARY KEY,
    book_id int NOT NULL REFERENCES books (id),
    name TEXT NOT NULL,
    comics TEXT[] NOT NULL
);

CREATE INDEX episode_book_id_idx ON episodes (book_id);

SELECT diesel_manage_updated_at('episodes');


CREATE TABLE episode_history (
    user_id int NOT NULL REFERENCES users (id),
    book_id int NOT NULL REFERENCES books (id),
    episode_id int NOT NULL REFERENCES episodes (id),
    PRIMARY KEY (user_id, book_id, episode_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('episode_history');

