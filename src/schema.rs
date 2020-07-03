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

table! {
    bands (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    questions (id) {
        id -> Uuid,
        text -> Varchar,
        correct_answer -> Uuid,
    }
}

table! {
    answers (id) {
        id -> Uuid,
        text -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (author_id));
joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
