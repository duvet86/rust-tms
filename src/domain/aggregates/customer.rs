#[derive(Clone, PartialEq, Eq)]
#[readonly::make]
pub struct Customer {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub address: String,
    pub contact_number: String,
}

impl Customer {
    pub fn new(id: u64, name: &str, email: &str, address: &str, contact_number: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            address: address.to_string(),
            contact_number: contact_number.to_string(),
        }
    }

    pub fn update(&mut self, name: &str, email: &str, address: &str, contact_number: &str) {
        self.name = name.to_string();
        self.email = email.to_string();
        self.address = address.to_string();
        self.contact_number = contact_number.to_string();
    }
}
