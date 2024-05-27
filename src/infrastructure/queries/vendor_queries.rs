use anyhow::Result;
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::models::vendor_dto::VendorDto;

pub async fn list_vendors(db_pool: PgPool) -> Result<Vec<VendorDto>> {
    let vendors = sqlx::query("SELECT * FROM vendors ORDER BY name")
        .map(|row: PgRow| VendorDto {
            id: row.get("id"),
            name: row.get("name"),
            address: row.get("address"),
            email: row.get("email"),
            contact_number: row.get("contact_number"),
        })
        .fetch_all(&db_pool)
        .await?;

    Ok(vendors)
}

pub async fn get_vendor_by_id(db_pool: PgPool, id: i32) -> Result<Option<VendorDto>> {
    let vendor = sqlx::query("SELECT * FROM vendors WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| VendorDto {
            id: row.get("id"),
            name: row.get("name"),
            address: row.get("address"),
            email: row.get("email"),
            contact_number: row.get("contact_number"),
        })
        .fetch_optional(&db_pool)
        .await?;

    Ok(vendor)
}
