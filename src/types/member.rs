use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroupMember {
    pub member_openid: String,
    pub username: String,
    pub member_role: MemberRole,
    pub bot: bool,
    pub joined_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_openid: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MemberRole {
    Admin,
    Member,
    Owner,
}
