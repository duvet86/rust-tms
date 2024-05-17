use crate::domain::aggregates::customer::Customer;
use anyhow::Result;

pub trait CustomerRepository {
    fn by_id(&self, id: u64) -> Result<Customer>;
    fn save(&self, client: Customer);
}
