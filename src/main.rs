use serde::Serialize;
use validator::{Validate, ValidateArgs};

fn main() {
    println!("Hello, world!");
}


#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(length(
        min = 3, 
        max = 15,
        // menambahkan attribute "message" atau pesan error
        message = "username must be between 3 and 15 character"
    ))]
    username: String,

    #[validate(length(
        min = 3, 
        max = 20,
        // menambahkan attribute "message" atau pesan error
        message = "password must be between 3 and 15 character"
    ))]
    password: String,
}


// Nested Struct
#[derive(Debug, Validate)]
struct AddressRequest {

    #[validate(length(min = 5, max = 100))]
    street: String,

    #[validate(length(min = 5, max = 100))]
    city: String,

    #[validate(length(min = 5, max = 100))]
    country: String,
}


#[derive(Debug, Validate)]
struct RegisterUserRequest {

    #[validate(length(min = 5, max = 20))]
    username: String,

    #[validate(length(min = 5, max = 20))]
    password: String,

    #[validate(length(min = 5, max = 100))]
    name: String,

    #[validate(nested)]
    address: AddressRequest,
}


// Collection
#[derive(Debug, Validate)]
struct Product {

    #[validate(length(min = 3, max = 100))]
    id: String,

    #[validate(length(min = 3, max = 100))]
    name: String,

    #[validate(nested, length(min = 1))]
    variants: Vec<ProductVariant>,
}

#[derive(Debug, Validate, Serialize)]
struct ProductVariant {

    #[validate(length(min = 3, max = 100))]
    name: String,

    #[validate(range(min = 1, max = 1000000000))]
    price: i32,
}


// contoh untuk validasi berhasil
#[test]
fn test_validate_success() {
    let login = LoginRequest {
        username: "suharjin07".to_string(),
        password: "12345".to_string()
    };

    assert!(login.validate().is_ok())
}

// contoh untuk validasi gagal
#[test]
fn test_validate_failed() {
    let login = LoginRequest {
        username: "c1".to_string(),
        password: "12".to_string()
    };

    assert!(login.validate().is_err());

    let error = login.validate().err().unwrap();
    println!("{:?}", error.errors());
}


// Contoh untuk validasi berhasil menggunakan Nested Struct
#[test]
fn test_nested_struct_success() {
    let request = RegisterUserRequest {
        username: "qwerty".to_string(),
        password: "12345".to_string(),
        name: "Suharjin".to_string(),
        address: AddressRequest {
            street: "St. Albert Einstain No. 18".to_string(),
            city: "Sidney".to_string(),
            country: "Australia".to_string()
        }
    };

    assert!(request.validate().is_ok())
}

// Contoh untuk validasi gagal menggunakan Nested Struct
#[test]
fn test_nested_struct_failed() {
    let request = RegisterUserRequest {
        username: "qwerty".to_string(), 
        password: "12345".to_string(),
        name: "Suharjin".to_string(),
        address: AddressRequest {
            street: "".to_string(), // street sengaja di kosongkan
            city: "Sidney".to_string(),
            country: "Australia".to_string()
        }
    };

    assert!(request.validate().is_err());

    let errors = request.validate().err().unwrap();
    println!("{:?}", errors.errors())
}


// Collection
#[test]
fn test_validate_vector_succes() {
    let request = Product {
        id: "12345".to_string(),
        name: "Product-1".to_string(),
        variants: vec![
            ProductVariant {
                name: "Variant-1".to_string(),
                price: 2000000
            },

            ProductVariant {
                name: "Variant-2".to_string(),
                price: 5000000
            },
        ]
    };

    assert!(request.validate().is_ok())
}

#[test]
fn test_validate_vector_failed() {
    let request = Product {
        id: "12345".to_string(),
        name: "Product-1".to_string(),
        variants: vec![
            ProductVariant {
                name: "".to_string(),
                price: 0
            },

            ProductVariant {
                name: "Variant-2".to_string(),
                price: 5000000
            },
        ]
    };

    assert!(request.validate().is_err());

    let errors = request.validate().err().unwrap();
    println!("{:?}", errors.errors())
}