use anyhow::Result;
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::models::customer_dto::CustomerDto;

pub async fn list_customers(db_pool: PgPool) -> Result<Vec<CustomerDto>> {
    let customers = sqlx::query("SELECT * FROM customers ORDER BY name")
        .map(|row: PgRow| CustomerDto {
            id: row.get("id"),
            name: row.get("name"),
            address: row.get("address"),
            email: row.get("email"),
            contact_number: row.get("contact_number"),
        })
        .fetch_all(&db_pool)
        .await?;

    Ok(customers)
}

pub async fn get_customer_by_id(db_pool: PgPool, id: i32) -> Result<Option<CustomerDto>> {
    let customer = sqlx::query("SELECT * FROM customers WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| CustomerDto {
            id: row.get("id"),
            name: row.get("name"),
            address: row.get("address"),
            email: row.get("email"),
            contact_number: row.get("contact_number"),
        })
        .fetch_optional(&db_pool)
        .await?;

    Ok(customer)
}
