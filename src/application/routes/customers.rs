use anyhow::Result;
use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::header::LOCATION, response::IntoResponse, routing::get, Router};
use http::{HeaderName, StatusCode};
use sqlx::PgPool;

use crate::application::utils::{app_state::AppState, http_utils::AppError};
use crate::domain::aggregates::customer::Customer;
use crate::infrastructure::queries::customer_queries::{get_customer_by_id, list_customers};
use crate::infrastructure::repositories::customer_repository::{CustomerRepository, Repository};
use crate::models::customer_dto::{CreateCustomerRequest, CustomerDto};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/customers",
            get(customers_list_handler).post(create_customer_handler),
        )
        .route(
            "/customers/:id",
            get(customer_handler).put(update_customer_handler),
        )
}

async fn customers_list_handler(
    State(db_pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let customers = list_customers(db_pool).await?;

    Ok(Json(customers))
}

async fn customer_handler(
    Path(id): Path<i32>,
    State(db_pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let customer = get_customer_by_id(db_pool, id).await?;

    match customer {
        Some(c) => Ok((StatusCode::OK, Json(c)).into_response()),
        None => Ok((StatusCode::NOT_FOUND).into_response()),
    }
}

async fn update_customer_handler(
    Path(id): Path<i32>,
    State(db_pool): State<PgPool>,
    Json(req): Json<CreateCustomerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let repo = CustomerRepository::new(db_pool);

    let mut customer = repo
        .by_id(id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND).into_response())
        .unwrap();

    customer.update(
        &req.name,
        &req.email,
        &req.address,
        req.contact_number.as_deref(),
    );

    repo.update(&customer).await?;

    let dto = CustomerDto {
        id: customer.id(),
        name: req.name,
        address: req.address,
        contact_number: req.contact_number,
        email: req.email,
    };

    Ok(Json(dto))
}

async fn create_customer_handler(
    State(db_pool): State<PgPool>,
    Json(req): Json<CreateCustomerRequest>,
) -> Result<
    (
        StatusCode,
        [(HeaderName, std::string::String); 1],
        axum::Json<CustomerDto>,
    ),
    AppError,
> {
    let repo = CustomerRepository::new(db_pool);

    let customer_domain = Customer::new(
        0,
        &req.name,
        &req.email,
        &req.address,
        req.contact_number.as_deref(),
    );

    let id = repo.create(&customer_domain).await?;

    let dto = CustomerDto {
        id,
        name: req.name,
        address: req.address,
        contact_number: req.contact_number,
        email: req.email,
    };

    let location_header = [(LOCATION, format!("/v1/api/customers/{}", id))];

    Ok((StatusCode::CREATED, location_header, Json(dto)))
}
