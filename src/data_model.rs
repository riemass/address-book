struct AddressBook {
    id: u64,
    name: String,
    description: String,
}

struct Contact {
    id: u64,
    name: String,
    surname: String,
}

struct Email {
    name: String,
    domain: String,
}

struct Address {
    street: String,
    postal_code: u32,
    city: String,
    country: String,
}

struct Phone {
    area_code: u32,
    prefix: u32,
    number: u32,
}
