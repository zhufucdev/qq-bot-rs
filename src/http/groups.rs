use async_stream::try_stream;
use futures_util::TryStream;
use serde::Deserialize;

use crate::{HttpError, http::Bot, types::member::GroupMember};

impl Bot {
    pub fn list_group_members(
        &self,
        group_openid: &str,
    ) -> impl TryStream<Ok = GroupMember, Error = HttpError> {
        try_stream! {
            let mut cursor: Option<Box<str>> = None;
            loop {
                let url = if let Some(cursor) = cursor.as_ref() {
                    format!("/v2/groups/{group_openid}/members?cursor={cursor}")
                } else {
                    format!("/v2/groups/{group_openid}/members")
                };
                let resp: MemberResponse = self.get_json(&url).await?;
                for member in resp.members {
                    yield member;
                }
                if let Some(next_cursor) = resp.next_cursor {
                    cursor = Some(next_cursor);
                } else {
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct MemberResponse {
    members: Box<[GroupMember]>,
    next_cursor: Option<Box<str>>,
}
