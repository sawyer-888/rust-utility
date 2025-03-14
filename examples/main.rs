use rust_utils::{math, string_utils, file_utils, thread_utils, net_utils, crypto_utils, serialization};

#[tokio::main]
async fn main() {
    println!("Factorial of 5: {}", math::factorial(5));
    println!("Is 'racecar' a palindrome? {}", string_utils::is_palindrome("racecar"));

    file_utils::write_file("example.txt", "Hello, Rust!").expect("Failed to write file");
    println!("File content: {:?}", file_utils::read_file("example.txt").unwrap());

    let url_content = net_utils::fetch_url("https://www.rust-lang.org").await.unwrap();
    println!("Fetched URL Content: {}", &url_content[..50]);

    println!("Hash of 'password123': {}", crypto_utils::hash_string("password123"));

    let config = serialization::Config {
        name: "RustApp".to_string(),
        version: "1.0.0".to_string(),
    };
    let json_data = serialization::json_serialize(&config);
    println!("Serialized JSON: {}", json_data);
}
