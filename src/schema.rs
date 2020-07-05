table! {
    answers (id) {
        id -> Uuid,
        content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    bands (id) {
        id -> Uuid,
        name -> Varchar,
        owner_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Uuid,
        author_id -> Uuid,
        post_id -> Uuid,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Uuid,
        author_id -> Uuid,
        slug -> Varchar,
        title -> Varchar,
        description -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    questions (id) {
        id -> Uuid,
        content -> Varchar,
        correct_answer_id -> Uuid,
        band_id -> Uuid,
    }
}

table! {
    todo_item (id) {
        id -> Int4,
        title -> Varchar,
        checked -> Bool,
        list_id -> Int4,
    }
}

table! {
    todo_list (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(bands -> users (owner_id));
joinable!(comments -> posts (post_id));
joinable!(comments -> users (author_id));
joinable!(posts -> users (author_id));
joinable!(questions -> answers (correct_answer_id));
joinable!(questions -> bands (band_id));
joinable!(todo_item -> todo_list (list_id));

allow_tables_to_appear_in_same_query!(
    answers,
    bands,
    comments,
    posts,
    questions,
    todo_item,
    todo_list,
    users,
);
