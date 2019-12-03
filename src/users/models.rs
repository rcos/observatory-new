use crate::auth::crypto::UnsafeBinaryString;
use crate::schema::*;
use chrono::NaiveDateTime;

/// User template
///
/// Represents the template for a user's page
#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Serialize)]
pub struct User {
    ///  ID of the user
    pub id: i32,
    /// Real name of the user
    pub real_name: String,
    /// Github user name of the user
    pub handle: String,
    /// Email of the user
    pub email: String,
    /// The encrypeted version of the user password
    #[serde(skip)]
    pub password_hash: UnsafeBinaryString,
    /// Randomly generated bianry number use to encrypt the password
    #[serde(skip)]
    pub salt: UnsafeBinaryString,
    /// The bio gives a brief summary of the user
    pub bio: String,
    /// active is a flag to let us know if a user
    pub active: bool,
    /// the date when they signed up for rcos
    pub joined_on: NaiveDateTime,
    /// Tier system lets us know if a user is a mentor, course coordinator, or admin
    pub tier: i32,
    /// Mattermost user name of the user
    pub mmost: String,
    /// flag used to mark if user that were students
    pub former: bool,
    /// flag used to mark if the user is not a student
    pub extrn: bool,
}

/// New User template
///
/// Represents the template for a when a new user is created
#[derive(Debug, Default, Clone, FromForm, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    /// Real name of the user
    pub real_name: String,
    /// Github user name of the user
    pub handle: String,
    /// The encryptd version of the user password
    pub password_hash: UnsafeBinaryString,
    /// randomly generated binary string
    pub salt: UnsafeBinaryString,
    /// Bio gives a brief summary of the user
    pub bio: String,
    /// Gives email of the user
    pub email: String,
    /// Tier system lets us know if a user is a mentor, course coordinator, or admin
    pub tier: i32,
    /// active is a flag to let us know if a user
    pub active: bool,
    /// The Mattermost handle of the user
    pub mmost: String,
    /// flag used to mark if user that were students
    pub former: bool,
    /// flag used to mark if the user is not a student
    pub extrn: bool,
}

use crate::models::Attendable;
/// GradeSummary template
///
/// Represents the template for a user's GradeSummary based on attendances and commits
#[derive(Debug, Default)]
pub struct GradeSummary {
    /// Tracks the users number of successfulo attendances
    pub attendances: Vec<Box<dyn Attendable>>,
    /// Tracks the number of total attendences including the missed ones
    pub needed_attendances: usize,
    /// Number of commits the user has made to a project
    pub commit_count: Option<usize>,
}
