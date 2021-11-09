table! {
    books (id) {
        id -> Int4,
        authors -> Array<Text>,
        slug -> Text,
        name -> Text,
        cover -> Text,
        description -> Text,
        tags -> Array<Text>,
        day_of_week -> Int4,
        favorites_count -> Int4,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    episode_historys (user_id, book_id, episode_id) {
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
        price -> Int4,
        comics -> Array<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
    reading_historys (user_id, book_id) {
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

joinable!(episode_historys -> books (book_id));
joinable!(episode_historys -> episodes (episode_id));
joinable!(episode_historys -> users (user_id));
joinable!(episodes -> books (book_id));
joinable!(favorite_books -> books (book_id));
joinable!(favorite_books -> users (user_id));
joinable!(reading_historys -> books (book_id));
joinable!(reading_historys -> users (user_id));

allow_tables_to_appear_in_same_query!(
    books,
    episode_historys,
    episodes,
    favorite_books,
    reading_historys,
    users,
);
