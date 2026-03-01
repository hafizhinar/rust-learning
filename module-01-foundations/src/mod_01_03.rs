const MAX_TRANSACTIONS: u32 = 100_000;
const APP_NAME: &str = "PaymentGateway";

pub fn run() {
    println!();
    println!("Modules 01.03 - Variables & Mutability");

    // 1. let - Immutable Variable
    println!();
    println!("1. let - Immutable Variable");
    let x = 5;
    println!("x = {}", x);

    // x = 10; // error karena immutable
    let mut x = 10;
    println!("x = {}", x);

    // 2. let mut - Mutable Variable
    println!();
    println!("2. let mut - Mutable Variable");
    let mut x = 5;
    println!("x awal = {}", x);
    x = 10;
    println!("x sekarang = {}", x);
    x = x + 5;
    println!("x akhir = {}", x);

    println!();
    println!("2.1. Contoh Use Case Real");
    let base_amount = 1000;
    let mut total = 0;
    let mut count = 0;
    for _ in 0..5 {
        total += base_amount;
        count += 1;
    }
    println!("Total: {}, count: {}", total, count);

    println!();
    println!("2.2. Analogi Payment System");
    process_transaction(5000.0);

    println!();
    println!("2.3. Type Inference vs Explicit Type");
    let x = 5;
    let y: i32 = 10;
    let z: f64 = std::f64::consts::PI;
    let name: &str = "Joni";
    let parsed: i32 = "42".parse().expect("Not a number");
    println!("Inference: {}, Explicit: {}, {}, {}, {}", x, y, z, name, parsed);

    println!();
    println!("2.4. Shadowing - Re-Declare Variable");
    let x = 5;
    println!("x pertama: {}", x);
    let x = x + 1;
    println!("x kedua: {}", x);
    let x = x * 2;
    println!("x ketiga: {}", x);

    println!();
    println!("2.5. Shadowing vs Mutation");

    println!("2.5.1. Shadowing - create new variable, tipe bisa berbeda");
    let x = "5";                       // &str
    let x = x.parse::<i32>().unwrap(); // i32
    let x = x * 2;                     // i32 = 10
    println!("x: {}", x);

    println!();
    println!("2.5.2. Mutation - harus tipe sama");
    let mut y: i32 = 5;
    y = 10;
    // y = "10"; // Error: tipe tidak bisa diganti
    println!("y adalah mutation tipe harus sama: {}", y);

    println!();
    println!("2.6. Shadowing untuk transform data");
    let input = " 150000 ";
    let input = input.trim();
    let input = input.parse::<i32>().unwrap();
    let input = input * 2;
    println!("Result: {}", input);

    // 3. const - Compile Time Constant
    println!();
    println!("3. const - Compile Time Constant");
    println!("MAX_TRANSACTIONS: {}", MAX_TRANSACTIONS);
    println!("APP: {}", APP_NAME);
    // const tidak bisa mutable:        const mut X: i32 = 5;          // error
    // const tidak bisa function call:  const NOW: u64 = get_timestamp(); // error

    let now = get_timestamp();
    println!("Get Timestamp: {}", now);
}

fn get_timestamp() -> u64 {
    1234567890
}

fn process_transaction(amount: f64) {
    let transaction_id = "TRX-001"; // immutable - ID tidak boleh berubah
    let mut balance = 10000.0;      // mutable - balance berubah setelah transaksi

    balance -= amount;

    // transaction_id = "TRX-002"; // Error
    println!("Transaction: {}, New Balance: {}", transaction_id, balance);
}

pub fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tambah() {
        assert_eq!(tambah(3, 5), 8);
    }
}
