//! # Torii Poem Integration
//!
//! This crate provides Poem extractors and middleware for the Torii authentication framework.
//!
//! ## Extractors
//!
//! - `AuthUser`: Extracts an authenticated user from request data (requires authentication)
//! - `OptionalAuthUser`: Extracts an optional user from request data (authentication not required)
//! - `SessionTokenFromCookie`: Extracts session token from cookie
//! - `SessionTokenFromBearer`: Extracts session token from Authorization Bearer header
//! - `SessionTokenFromRequest`: Extracts session token from either Bearer or cookie (Bearer takes precedence)
//!
//! ## Middleware
//!
//! - `AuthMiddleware`: Optional authentication that loads user into request data if valid session exists
//! - `RequireAuthMiddleware`: Required authentication that returns 401 if no valid session exists
//!
//! ## Example Usage
//!
//! ### Using Extractors
//!
//! ```rust,no_run
//! use poem::{handler, web::Json};
//! use torii_poem::AuthUser;
//! use torii::User;
//!
//! #[handler]
//! async fn get_profile(AuthUser(user): AuthUser) -> Json<User> {
//!     Json(user)
//! }
//! ```
//!
//! ### Using Middleware
//!
//! ```rust,no_run
//! use poem::{Route, get, handler};
//! use torii_poem::{AuthMiddleware, RequireAuthMiddleware, AuthUser};
//! use std::sync::Arc;
//!
//! #[handler]
//! async fn protected(AuthUser(user): AuthUser) -> String {
//!     format!("Hello, {}!", user.email)
//! }
//!
//! let torii = Arc::new(/* your Torii instance */);
//!
//! // Optional auth - user will be available if session exists
//! let with_optional_auth = Route::new()
//!     .at("/profile", get(protected))
//!     .with(AuthMiddleware::new(torii.clone()));
//!
//! // Required auth - returns 401 if no session
//! let with_required_auth = Route::new()
//!     .at("/admin", get(protected))
//!     .with(RequireAuthMiddleware::new(torii));
//! ```

pub mod extractor;
pub mod middleware;

pub use extractor::{
    AuthUser, OptionalAuthUser, SessionTokenFromBearer, SessionTokenFromCookie,
    SessionTokenFromRequest,
};
pub use middleware::{AuthMiddleware, RequireAuthMiddleware};
