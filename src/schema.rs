table! {
    groups (id) {
        id -> Integer,
        name -> Text,
        owner_id -> Integer,
        room -> Nullable<Text>,
    }
}

table! {
    meetings (id) {
        id -> Integer,
        datetime -> Integer,
        code -> Text,
        group_id -> Integer,
    }
}

table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        homepage -> Nullable<Text>,
        repo -> Text,
        owner_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        real_name -> Text,
        handle -> Text,
        email -> Text,
        password_hash -> Text,
        active -> Bool,
        joined_on -> Integer,
        tier -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    groups,
    meetings,
    projects,
    users,
);
