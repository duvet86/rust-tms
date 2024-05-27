use anyhow::Result;
use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::header::LOCATION, response::IntoResponse, routing::get, Router};
use http::{HeaderName, StatusCode};
use sqlx::PgPool;

use crate::application::utils::{app_state::AppState, http_utils::AppError};
use crate::domain::aggregates::vendor::Vendor;
use crate::infrastructure::queries::vendor_queries::{get_vendor_by_id, list_vendors};
use crate::infrastructure::repositories::vendor_repository::{Repository, VendorRepository};
use crate::models::vendor_dto::{CreateVendorRequest, VendorDto};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/vendors",
            get(vendors_list_handler).post(create_vendor_handler),
        )
        .route(
            "/vendors/:id",
            get(vendor_handler).put(update_vendor_handler),
        )
}

async fn vendors_list_handler(
    State(db_pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let vendors = list_vendors(db_pool).await?;

    Ok(Json(vendors))
}

async fn vendor_handler(
    Path(id): Path<i32>,
    State(db_pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let vendor = get_vendor_by_id(db_pool, id).await?;

    match vendor {
        Some(c) => Ok((StatusCode::OK, Json(c)).into_response()),
        None => Ok((StatusCode::NOT_FOUND).into_response()),
    }
}

async fn update_vendor_handler(
    Path(id): Path<i32>,
    State(db_pool): State<PgPool>,
    Json(req): Json<CreateVendorRequest>,
) -> Result<impl IntoResponse, AppError> {
    let repo = VendorRepository::new(db_pool);

    let mut vendor = repo
        .by_id(id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND).into_response())
        .unwrap();

    vendor.update(
        &req.name,
        &req.email,
        &req.address,
        req.contact_number.as_deref(),
    );

    repo.update(&vendor).await?;

    let dto = VendorDto {
        id: vendor.id(),
        name: req.name,
        address: req.address,
        contact_number: req.contact_number,
        email: req.email,
    };

    Ok(Json(dto))
}

async fn create_vendor_handler(
    State(db_pool): State<PgPool>,
    Json(req): Json<CreateVendorRequest>,
) -> Result<
    (
        StatusCode,
        [(HeaderName, std::string::String); 1],
        axum::Json<VendorDto>,
    ),
    AppError,
> {
    let repo = VendorRepository::new(db_pool);

    let vendor_domain = Vendor::new(
        0,
        &req.name,
        &req.email,
        &req.address,
        req.contact_number.as_deref(),
    );

    let id = repo.create(&vendor_domain).await?;

    let dto = VendorDto {
        id,
        name: req.name,
        address: req.address,
        contact_number: req.contact_number,
        email: req.email,
    };

    let location_header = [(LOCATION, format!("/v1/api/vendors/{}", id))];

    Ok((StatusCode::CREATED, location_header, Json(dto)))
}
