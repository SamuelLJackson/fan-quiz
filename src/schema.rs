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
    questions (id) {
        id -> Uuid,
        content -> Varchar,
        correct_answer_id -> Uuid,
        band_id -> Uuid,
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

joinable!(bands -> users (owner_id));
joinable!(questions -> answers (correct_answer_id));
joinable!(questions -> bands (band_id));

allow_tables_to_appear_in_same_query!(
    answers,
    bands,
    questions,
    users,
);
