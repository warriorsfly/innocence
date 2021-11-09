
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    authors TEXT[] NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    cover TEXT NOT NULL,
    description TEXT NOT NULL,
    tags TEXT[] NOT NULL,
    -- Mon,Tue,Wed,Thu,Fri,Sat,Sun
    day_of_week int not null,
    favorites_count int NOT NULL DEFAULT 0,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX books_authors_idx ON books (authors);
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

CREATE TABLE reading_historys (
    user_id int NOT NULL REFERENCES users (id),
    book_id int NOT NULL REFERENCES books (id),
    PRIMARY KEY (user_id, book_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX reading_books_user_id_idx ON favorite_books (user_id);
CREATE INDEX reading_books_book_id_idx ON favorite_books (book_id);

SELECT diesel_manage_updated_at('reading_historys');

CREATE TABLE episodes (
    id SERIAL PRIMARY KEY,
    book_id int NOT NULL REFERENCES books (id),
    name TEXT NOT NULL,
    price int NOT NULL DEFAULT 0,
    comics TEXT[] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX episode_book_id_idx ON episodes (book_id);
SELECT diesel_manage_updated_at('episodes');

CREATE TABLE episode_historys (
    user_id int NOT NULL REFERENCES users (id),
    book_id int NOT NULL REFERENCES books (id),
    episode_id int NOT NULL REFERENCES episodes (id),
    PRIMARY KEY (user_id, book_id, episode_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('episode_historys');

