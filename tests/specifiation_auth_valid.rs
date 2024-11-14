
/* TODO: Auth messages not supported
#[test]
#[ignore = "not yet implemented"]
fn login_request() {
    let path = Path::new("tests/specification/test_data/auth-valid/login-request.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Auth = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn login_response() {
    let path = Path::new("tests/specification/test_data/auth-valid/login-response.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Auth = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn logout_request() {
    let path = Path::new("tests/specification/test_data/auth-valid/logout-request.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Auth = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn validate_request() {
    let path = Path::new("tests/specification/test_data/auth-valid/validate-request.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Auth = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn validate_response() {
    let path = Path::new("tests/specification/test_data/auth-valid/validate-response.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Auth = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/
