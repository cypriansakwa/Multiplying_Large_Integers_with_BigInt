// Import necessary crates
use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn main() {
    // Create BigInt values from integers
    let a = BigInt::from_i64(30001483).unwrap();
    let b = BigInt::from_i64(30000521).unwrap();

    // Perform multiplication
    let result = &a * &b;

    // Print the result
    println!("{} * {} = {}", a, b, result);
}
