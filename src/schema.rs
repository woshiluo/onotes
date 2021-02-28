table! {
    histories (id) {
        id -> Integer,
        post_id -> Integer,
        time -> Integer,
        markdown -> Nullable<Text>,
    }
}

table! {
    post_edge (id) {
        id -> Integer,
        from_post -> Integer,
        to_post -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        markdown -> Nullable<Text>,
    }
}

table! {
    tokens (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        nickname -> Text,
        password -> Text,
        email -> Text,
        admin -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    histories,
    post_edge,
    posts,
    tokens,
    users,
);
