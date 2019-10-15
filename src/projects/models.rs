use crate::schema::*;

use crate::models::User;

#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
    pub active: bool,
    pub repos: String,
    pub extrn: bool,
}

#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub owner_id: i32,
    pub repos: String,
    pub extrn: bool,
}

#[derive(Debug, PartialEq, Clone, Queryable, Associations, Identifiable)]
#[table_name = "relation_project_user"]
#[belongs_to(Project)]
#[belongs_to(User)]
pub struct RelationProjectUser {
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Default, Clone, Insertable)]
#[table_name = "relation_project_user"]
pub struct NewRelationProjectUser {
    pub project_id: i32,
    pub user_id: i32,
}
