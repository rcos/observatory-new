use crate::schema::*;

use crate::models::User;

/// Project template
///
/// Represents the template for a project page
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct Project {
    /// ID of the Project
    pub id: i32,
    /// Name of the Project
    pub name: String,
    /// Description of the Project
    pub description: String,
    /// Link if the project has their own website
    pub homepage: Option<String>,
    /// ID of the student that owns the project
    pub owner_id: i32,
    /// Checks if this is a project that is currently being worked on this semester
    pub active: bool,
    /// Link the Project repository
    pub repos: String,
    pub extrn: bool,
}

/// New Project Template
///
/// Represents the template when a new project is created
#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "projects"]
pub struct NewProject {
    /// Name of the Project
    pub name: String,
    /// Description of the Project
    pub description: String,
    /// Puts in URL for the projects website if it has one
    pub homepage: Option<String>,
    /// The ID of the student who creates and owns the new project
    pub owner_id: i32,
    /// Link to the Project Repository
    pub repos: String,
    pub extrn: bool,
}

/// Student Relation to the Project
///
/// Represents the data checking a students ties to a project
#[derive(Debug, PartialEq, Clone, Queryable, Associations, Identifiable)]
#[table_name = "relation_project_user"]
#[belongs_to(Project)]
#[belongs_to(User)]
pub struct RelationProjectUser {
    /// Represents the table of user IDs tied to the project
    pub id: i32,
    /// The ID of the Project
    pub project_id: i32,
    /// The ID of the Indivual User
    pub user_id: i32,
}

/// Used to Tie a student to a project
#[derive(Debug, Default, Clone, Insertable)]
#[table_name = "relation_project_user"]
pub struct NewRelationProjectUser {
    /// ID of the Project
    pub project_id: i32,
    /// ID of the student being added to the project
    pub user_id: i32,
}
