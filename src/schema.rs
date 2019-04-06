table! {
    events (id) {
        id -> Integer,
        happening_at -> Timestamp,
        title -> Text,
        description -> Nullable<Text>,
        hosted_by -> Integer,
        location -> Nullable<Text>,
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
    projects (id) {
        id -> Integer,
        name -> Text,
        homepage -> Nullable<Text>,
        owner_id -> Integer,
        active -> Bool,
    }
}

table! {
    relation_group_user (id) {
        id -> Integer,
        group_id -> Integer,
        member_id -> Integer,
    }
}

table! {
    relation_meeting_user (id) {
        id -> Integer,
        meeting_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    relation_project_user (id) {
        id -> Integer,
        project_ID -> Integer,
        user_id -> Integer,
    }
}

table! {
    repos (id) {
        id -> Integer,
        project_id -> Integer,
        url -> Text,
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
        active -> Bool,
        joined_on -> Timestamp,
        tier -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    groups,
    meetings,
    projects,
    relation_group_user,
    relation_meeting_user,
    relation_project_user,
    repos,
    users,
);
