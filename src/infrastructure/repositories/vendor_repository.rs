use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use sqlx::postgres::PgPool;

use crate::domain::aggregates::vendor::Vendor;

pub struct VendorRepository {
    pg_pool: Arc<PgPool>,
}

impl VendorRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pg_pool),
        }
    }
}

#[async_trait]
pub trait Repository {
    async fn by_id(&self, id: i32) -> Result<Vendor>;
    async fn create<'a, 'b>(&'a self, vendor: &'b Vendor) -> Result<i32>;
    async fn update<'a, 'b>(&'a self, vendor: &'b Vendor) -> Result<bool>;
}

#[async_trait]
impl Repository for VendorRepository {
    async fn by_id(&self, id: i32) -> Result<Vendor> {
        let vendor_db = sqlx::query!(
            r#"
        SELECT id, name, email, address, contact_number
        FROM vendors
        WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(Vendor::new(
            vendor_db.id,
            vendor_db.name.as_str(),
            vendor_db.email.as_str(),
            vendor_db.address.as_str(),
            vendor_db.contact_number.as_deref(),
        ))
    }

    async fn create<'a, 'b>(&'a self, vendor: &'b Vendor) -> Result<i32> {
        match vendor.id() {
            value if value != 0 => panic!("Vendor id must be 0."),
            _ => (),
        }

        let record = sqlx::query!(
            r#"
INSERT INTO vendors (name, email, address, contact_number)
VALUES ($1, $2, $3, $4)
RETURNING id
        "#,
            vendor.name,
            vendor.email,
            vendor.address,
            vendor.contact_number
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(record.id)
    }

    async fn update<'a, 'b>(&'a self, vendor: &'b Vendor) -> Result<bool> {
        match vendor.id() {
            value if value == 0 => panic!("Vendor id cannot be 0."),
            _ => (),
        }

        let rows_affected = sqlx::query!(
            r#"
UPDATE vendors SET name = $1, email = $2, address = $3, contact_number = $4
WHERE id = $5
        "#,
            vendor.name,
            vendor.email,
            vendor.address,
            vendor.contact_number,
            vendor.id
        )
        .execute(&*self.pg_pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}
