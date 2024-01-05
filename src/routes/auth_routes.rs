use crate::{
    http::ApiContext,
    structs::{LoginRequest, LoginResponse, RegisterRequest, User},
};

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::post,
    Extension, Form, Json, Router,
};
use bcrypt::DEFAULT_COST;
use sqlx::{Error, PgPool};

pub fn get_routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn login(
    ctx: Extension<ApiContext>,
    Form(payload): Form<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = match get_user(&payload.username, &payload.password, &ctx.db).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Error getting user: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(user))
}

async fn get_user(username: &str, password: &str, pool: &PgPool) -> Result<LoginResponse, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password FROM users WHERE name = $1",
        username,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::RowNotFound,
        _ => e.into(),
    })?;

    let pswd = user.password.ok_or(Error::RowNotFound)?;
    bcrypt::verify(password, &pswd).map_err(|_| Error::RowNotFound)?;

    let name = user.name.ok_or(Error::RowNotFound)?;
    let email = user.email.ok_or(Error::RowNotFound)?;

    Ok(LoginResponse {
        id: user.id,
        name,
        email,
    })
}

async fn register(ctx: Extension<ApiContext>, Form(payload): Form<RegisterRequest>) -> StatusCode {
    if let Err(err) = insert_user(&payload, &ctx.db).await {
        eprintln!("Error adding user: {}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}

async fn insert_user(payload: &RegisterRequest, pool: &PgPool) -> Result<Response<Body>, Error> {
    // VALIDATIONS
    if payload.password != payload.password_confirm {
        return Err(Error::RowNotFound);
    }
    // Hash the password
    let hashed_password = match bcrypt::hash(&payload.password, DEFAULT_COST) {
        Ok(hashed_password) => hashed_password,
        Err(_) => return Err(Error::RowNotFound), // or handle the error in a way that suits your needs
    };
    // Insert
    let _inserted_user = sqlx::query(
        "INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id",
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&hashed_password)
    .execute(pool)
    .await?;
    Ok(Redirect::to("/dashboard").into_response())
}
