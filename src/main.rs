use validator::Validate;

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