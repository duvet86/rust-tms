#[derive(Clone, PartialEq, Eq, Debug)]
#[readonly::make]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub address: String,
    pub contact_number: Option<String>,
}

impl Customer {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn new(
        id: i32,
        name: &str,
        email: &str,
        address: &str,
        contact_number: Option<&str>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            address: address.to_string(),
            contact_number: match contact_number {
                None => None,
                Some(str) => Some(str.to_string()),
            },
        }
    }

    pub fn update(&mut self, name: &str, email: &str, address: &str, contact_number: Option<&str>) {
        self.name = name.to_string();
        self.email = email.to_string();
        self.address = address.to_string();
        self.contact_number = match contact_number {
            None => None,
            Some(str) => Some(str.to_string()),
        }
    }
}
