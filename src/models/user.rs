use async_session::{MemoryStore, SessionStore};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    RequestPartsExt,
};
use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
use http::{
    header::{self},
    request::Parts,
};
use serde::{Deserialize, Serialize};
use tsm::{AuthRedirect, COOKIE_NAME};

// The user data we'll get back from Microsoft Graph.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    pub surname: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    pub id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}
