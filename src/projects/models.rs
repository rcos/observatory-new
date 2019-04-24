use crate::schema::*;

use crate::users::models::User;

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
    pub active: bool,
    pub repos: String,
}

#[derive(Debug, Default, FromForm, Insertable, AsChangeset)]
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
    pub repos: String,
}

#[derive(Debug, PartialEq, Queryable, Associations, Identifiable)]
#[table_name = "relation_project_user"]
#[belongs_to(Project)]
#[belongs_to(User)]
pub struct RelationProjectUser {
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "relation_project_user"]
pub struct NewRelationProjectUser {
    pub project_id: i32,
    pub user_id: i32,
}
