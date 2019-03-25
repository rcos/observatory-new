table! {
    attendance (id) {
        id -> Nullable<Integer>,
        datetime -> Integer,
        code -> Text,
        group_id -> Integer,
    }
}

table! {
    groups (id) {
        id -> Nullable<Integer>,
        name -> Text,
        owner_id -> Integer,
        room -> Nullable<Text>,
    }
}

table! {
    projects (id) {
        id -> Nullable<Integer>,
        name -> Text,
        homepage -> Nullable<Text>,
        repo -> Text,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
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
    attendance,
    groups,
    projects,
    users,
);
