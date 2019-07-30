table! {
    attendances (id) {
        id -> Integer,
        is_event -> Bool,
        user_id -> Integer,
        meeting_id -> Nullable<Integer>,
        event_id -> Nullable<Integer>,
    }
}

table! {
    events (id) {
        id -> Integer,
        start -> Timestamp,
        end -> Timestamp,
        title -> Text,
        description -> Nullable<Text>,
        hosted_by -> Integer,
        location -> Nullable<Text>,
        code -> Text,
        color -> Nullable<Text>,
    }
}

table! {
    groups (id) {
        id -> Integer,
        name -> Text,
        owner_id -> Integer,
        location -> Nullable<Text>,
    }
}

table! {
    meetings (id) {
        id -> Integer,
        happened_at -> Timestamp,
        code -> Text,
        group_id -> Integer,
        hosted_by -> Integer,
    }
}

table! {
    news (id) {
        id -> Integer,
        happened_at -> Timestamp,
        title -> Text,
        description -> Text,
        color -> Nullable<Text>,
    }
}

table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        homepage -> Nullable<Text>,
        owner_id -> Integer,
        active -> Bool,
        repos -> Text,
    }
}

table! {
    relation_group_user (id) {
        id -> Integer,
        group_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    relation_project_user (id) {
        id -> Integer,
        project_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        real_name -> Text,
        handle -> Text,
        email -> Text,
        password_hash -> Text,
        salt -> Text,
        bio -> Text,
        active -> Bool,
        joined_on -> Timestamp,
        tier -> Integer,
        mmost -> Text,
    }
}

joinable!(attendances -> events (event_id));
joinable!(attendances -> meetings (meeting_id));
joinable!(attendances -> users (user_id));
joinable!(relation_group_user -> groups (group_id));
joinable!(relation_group_user -> users (user_id));
joinable!(relation_project_user -> projects (project_id));
joinable!(relation_project_user -> users (user_id));

allow_tables_to_appear_in_same_query!(
    attendances,
    events,
    groups,
    meetings,
    news,
    projects,
    relation_group_user,
    relation_project_user,
    users,
);
