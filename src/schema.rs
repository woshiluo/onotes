table! {
    histories (id) {
        id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        time -> Unsigned<Integer>,
        markdown -> Nullable<Text>,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Integer>,
        title -> Text,
        markdown -> Nullable<Text>,
    }
}

table! {
    post_edge (id) {
        id -> Unsigned<Integer>,
        from_post -> Unsigned<Integer>,
        to_post -> Unsigned<Integer>,
    }
}

table! {
    tokens (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        token -> Text,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        nickname -> Text,
        password -> Text,
        email -> Text,
        admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    histories,
    posts,
    post_edge,
    tokens,
    users,
);
