use std::sync::Arc;

use poem::{Endpoint, FromRequest, Middleware, Request, Result as PoemResult};
use torii::Torii;
use torii_core::RepositoryProvider;

use crate::extractor::SessionTokenFromRequest;

/// Optional authentication middleware that loads user from session if present
///
/// This middleware:
/// - Extracts session token from Bearer header or cookie
/// - Validates the session and loads the user
/// - Inserts the user into request data if found
/// - Does NOT block the request if no valid session exists
///
/// # Example
///
/// ```rust,no_run
/// use poem::{Route, endpoint::make_sync, get};
/// use torii_poem::AuthMiddleware;
/// use std::sync::Arc;
///
/// let torii = Arc::new(/* your Torii instance */);
/// let route = Route::new()
///     .at("/", get(handler))
///     .with(AuthMiddleware::new(torii));
/// ```
pub struct AuthMiddleware<R: RepositoryProvider> {
    torii: Arc<Torii<R>>,
}

impl<R: RepositoryProvider> AuthMiddleware<R> {
    pub fn new(torii: Arc<Torii<R>>) -> Self {
        Self { torii }
    }
}

impl<E: Endpoint, R: RepositoryProvider + 'static> Middleware<E> for AuthMiddleware<R> {
    type Output = AuthMiddlewareImpl<E, R>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthMiddlewareImpl {
            ep,
            torii: self.torii.clone(),
        }
    }
}

pub struct AuthMiddlewareImpl<E, R: RepositoryProvider> {
    ep: E,
    torii: Arc<Torii<R>>,
}

impl<E: Endpoint, R: RepositoryProvider + 'static> Endpoint for AuthMiddlewareImpl<E, R> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> PoemResult<Self::Output> {
        // Use the extractor to get the session token
        let (mut req, mut body) = req.split();
        let SessionTokenFromRequest(session_token) =
            SessionTokenFromRequest::from_request(&req, &mut body).await?;

        if let Some(session_token) = session_token {
            match self.torii.get_session(&session_token).await {
                Ok(session) => match self.torii.get_user(&session.user_id).await {
                    Ok(Some(user)) => {
                        req.extensions_mut().insert(user);
                    }
                    Ok(None) => {
                        tracing::warn!("User not found for session: {:?}", session.user_id);
                    }
                    Err(e) => {
                        tracing::error!("Error getting user: {:?}", e);
                    }
                },
                Err(e) => {
                    tracing::debug!("Invalid session: {:?}", e);
                }
            }
        }

        self.ep.call(req).await
    }
}

/// Required authentication middleware that blocks requests without valid sessions
///
/// This middleware:
/// - Extracts session token from Bearer header or cookie
/// - Validates the session exists
/// - Returns 401 Unauthorized if no valid session is found
/// - Allows the request to proceed if session is valid
///
/// # Example
///
/// ```rust,no_run
/// use poem::{Route, endpoint::make_sync, get};
/// use torii_poem::RequireAuthMiddleware;
/// use std::sync::Arc;
///
/// let torii = Arc::new(/* your Torii instance */);
/// let route = Route::new()
///     .at("/protected", get(handler))
///     .with(RequireAuthMiddleware::new(torii));
/// ```
pub struct RequireAuthMiddleware<R: RepositoryProvider> {
    torii: Arc<Torii<R>>,
}

impl<R: RepositoryProvider> RequireAuthMiddleware<R> {
    pub fn new(torii: Arc<Torii<R>>) -> Self {
        Self { torii }
    }
}

impl<E: Endpoint, R: RepositoryProvider + 'static> Middleware<E> for RequireAuthMiddleware<R> {
    type Output = RequireAuthMiddlewareImpl<E, R>;

    fn transform(&self, ep: E) -> Self::Output {
        RequireAuthMiddlewareImpl {
            ep,
            torii: self.torii.clone(),
        }
    }
}

pub struct RequireAuthMiddlewareImpl<E, R: RepositoryProvider> {
    ep: E,
    torii: Arc<Torii<R>>,
}

impl<E: Endpoint, R: RepositoryProvider + 'static> Endpoint for RequireAuthMiddlewareImpl<E, R> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> PoemResult<Self::Output> {
        use poem::{error::Error as PoemError, http::StatusCode};

        // Use the extractor to get the session token
        let (req, mut body) = req.split();
        let SessionTokenFromRequest(session_token) =
            SessionTokenFromRequest::from_request(&req, &mut body).await?;

        let session_token =
            session_token.ok_or_else(|| PoemError::from_status(StatusCode::UNAUTHORIZED))?;

        // Validate that the session exists
        self.torii
            .get_session(&session_token)
            .await
            .map_err(|_| PoemError::from_status(StatusCode::UNAUTHORIZED))?;

        self.ep.call(req).await
    }
}
