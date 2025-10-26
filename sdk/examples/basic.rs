//! Basic usage example
//! Run with: cargo run --example basic --features std

#[cfg(feature = "std")]
fn main() {
    use polyendpoint_sdk::contract::*;

    println!("=== PolyEndpoint SDK Examples ===\n");

    // Example 1: Encode addEndpoint transaction
    let tx = encode_add_endpoint("https://api.example.com");
    println!("1. Add Endpoint:");
    println!("   Endpoint: https://api.example.com");
    println!("   Hex: {}\n", tx.to_hex());

    // Example 2: Encode removeEndpoint transaction
    let tx = encode_remove_endpoint("https://api.example.com");
    println!("2. Remove Endpoint:");
    println!("   Endpoint: https://api.example.com");
    println!("   Hex: {}\n", tx.to_hex());

    // Example 3: Encode addAdmin transaction
    let admin = "0x1234567890abcdef1234567890abcdef12345678";
    match encode_add_admin(admin) {
        Ok(tx) => {
            println!("3. Add Admin:");
            println!("   Admin: {}", admin);
            println!("   Hex: {}\n", tx.to_hex());
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 4: Read-only calls
    let tx = encode_get_endpoint_count();
    println!("4. Get Endpoint Count:");
    println!("   Hex: {}\n", tx.to_hex());

    let tx = encode_get_endpoint(0);
    println!("5. Get Endpoint by Index:");
    println!("   Index: 0");
    println!("   Hex: {}\n", tx.to_hex());

    let tx = encode_has_endpoint("https://api.example.com");
    println!("6. Check Endpoint Exists:");
    println!("   Endpoint: https://api.example.com");
    println!("   Hex: {}\n", tx.to_hex());

    println!("✓ All examples completed!");
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' feature. Run with:");
    println!("cargo run --example basic --features std");
}

