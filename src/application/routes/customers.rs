use anyhow::Result;
use axum::Json;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::application::utils::{app_state::AppState, http_utils::AppError};
use crate::domain::aggregates::customer::Customer;
use crate::infrastructure::repositories::customer_repository::{CustomerRepository, Repository};
use crate::models::customer_dto::{CreateCustomerRequest, CustomerDto};
use crate::models::user_dto::UserDto;

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/customers",
        get(customers_list_handler).post(create_customer_handler),
    )
}

async fn customers_list_handler(
    _: UserDto,
    State(db_pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let customers = sqlx::query("SELECT * FROM customers")
        .map(|row: PgRow| CustomerDto {
            id: row.get("id"),
            name: row.get("name"),
            address: row.get("address"),
            email: row.get("email"),
            contact_number: row.get("contact_number"),
        })
        .fetch_all(&db_pool)
        .await?;

    Ok(Json(customers))
}

async fn create_customer_handler(
    _: UserDto,
    State(db_pool): State<PgPool>,
    Json(req): Json<CreateCustomerRequest>,
) -> Result<Json<CustomerDto>, AppError> {
    let repo = CustomerRepository::new(db_pool);

    let customer_domain = Customer::new(
        0,
        &req.name,
        &req.email,
        &req.address,
        req.contact_number.as_deref(),
    );

    let id = repo.create(customer_domain).await?;

    let dto = CustomerDto {
        id,
        name: req.name,
        address: req.address,
        contact_number: req.contact_number,
        email: req.email,
    };

    Ok(Json(dto))
}
