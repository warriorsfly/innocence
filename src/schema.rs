table! {
    books (id) {
        id -> Int4,
        author_id -> Int4,
        slug -> Text,
        name -> Text,
        cover -> Text,
        description -> Text,
        body -> Text,
        tags -> Array<Text>,
        favorites_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    episode_history (user_id, book_id, episode_id) {
        user_id -> Int4,
        book_id -> Int4,
        episode_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    episodes (id) {
        id -> Int4,
        book_id -> Int4,
        name -> Text,
        comics -> Array<Text>,
    }
}

table! {
    favorite_books (user_id, book_id) {
        user_id -> Int4,
        book_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Varchar,
        password -> Text,
        bio -> Text,
        avatar -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
joinable!(books -> users (author_id));
joinable!(episode_history -> books (book_id));
joinable!(episode_history -> episodes (episode_id));
joinable!(episode_history -> users (user_id));
joinable!(episodes -> books (book_id));
joinable!(favorite_books -> books (book_id));
joinable!(favorite_books -> users (user_id));

allow_tables_to_appear_in_same_query!(books, episode_history, episodes, favorite_books, users,);
