use std::sync::Arc;

use crate::domain::aggregates::customer::Customer;
use anyhow::Result;
use axum::async_trait;
use sqlx::postgres::PgPool;

pub struct CustomerRepository {
    pg_pool: Arc<PgPool>,
}

impl CustomerRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pg_pool),
        }
    }
}

#[async_trait]
pub trait Repository {
    async fn by_id(&self, id: i32) -> Result<Customer>;
    async fn create(&self, customer: Customer) -> Result<i32>;
    async fn update(&self, customer: Customer) -> Result<bool>;
}

#[async_trait]
impl Repository for CustomerRepository {
    async fn by_id(&self, id: i32) -> Result<Customer> {
        let customer_db = sqlx::query!(
            r#"
        SELECT id, name, email, address, contact_number
        FROM customers
        WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(Customer::new(
            customer_db.id,
            customer_db.name.as_str(),
            customer_db.email.as_str(),
            customer_db.address.as_str(),
            customer_db.contact_number.as_deref(),
        ))
    }

    async fn create(&self, customer: Customer) -> Result<i32> {
        let record = sqlx::query!(
            r#"
INSERT INTO customers (name, email, address, contact_number)
VALUES ($1, $2, $3, $4)
RETURNING id
        "#,
            customer.name,
            customer.email,
            customer.address,
            customer.contact_number
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(record.id)
    }

    async fn update(&self, customer: Customer) -> Result<bool> {
        let rows_affected = sqlx::query!(
            r#"
UPDATE customers SET name = $1, email = $2, address = $3, contact_number = $4
WHERE id = $5
        "#,
            customer.name,
            customer.email,
            customer.address,
            customer.contact_number,
            customer.id
        )
        .execute(&*self.pg_pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}
