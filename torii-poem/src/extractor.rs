use poem::{
    Error as PoemError, FromRequest, Request, RequestBody, Result as PoemResult, http::StatusCode,
};
use torii::{SessionToken, User};

/// Authenticated user extractor - requires authentication middleware to have set the user
/// Will return 401 Unauthorized if no user is present
pub struct AuthUser(pub User);

impl<'a> FromRequest<'a> for AuthUser {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> PoemResult<Self> {
        let user = req
            .data::<User>()
            .ok_or_else(|| PoemError::from_status(StatusCode::UNAUTHORIZED))?;

        Ok(AuthUser(user.clone()))
    }
}

/// Optional authenticated user extractor - does not require authentication
/// Returns None if no user is present in request data
pub struct OptionalAuthUser(pub Option<User>);

impl<'a> FromRequest<'a> for OptionalAuthUser {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> PoemResult<Self> {
        let user = req.data::<User>().cloned();
        Ok(OptionalAuthUser(user))
    }
}

/// Session token extracted from cookie named "session_id"
pub struct SessionTokenFromCookie(pub Option<SessionToken>);

impl<'a> FromRequest<'a> for SessionTokenFromCookie {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> PoemResult<Self> {
        let session_token = req
            .cookie()
            .get("session_id")
            .map(|cookie| SessionToken::new(cookie.value_str()));

        Ok(SessionTokenFromCookie(session_token))
    }
}

/// Session token extracted from Authorization Bearer header
pub struct SessionTokenFromBearer(pub Option<SessionToken>);

impl<'a> FromRequest<'a> for SessionTokenFromBearer {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> PoemResult<Self> {
        let session_token = req
            .headers()
            .get("authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(SessionToken::new);

        Ok(SessionTokenFromBearer(session_token))
    }
}

/// Session token extracted from either Bearer token or cookie
/// Tries Bearer token first, then falls back to cookie
pub struct SessionTokenFromRequest(pub Option<SessionToken>);

impl<'a> FromRequest<'a> for SessionTokenFromRequest {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> PoemResult<Self> {
        // Try Bearer token first
        if let Some(token) = req
            .headers()
            .get("authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
        {
            return Ok(SessionTokenFromRequest(Some(SessionToken::new(token))));
        }

        // Fall back to cookie
        let session_token = req
            .cookie()
            .get("session_id")
            .map(|cookie| SessionToken::new(cookie.value_str()));

        Ok(SessionTokenFromRequest(session_token))
    }
}
