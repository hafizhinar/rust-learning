fn main() {
    // modules 01.01
    println!("Hello, World!");

    //modules 01.02
    // Basic Macro
    // 1. println! - print new line
    println!("Hello, Rust"); // normal teks
    println!("Nilai: {}", 42); // satu variabel
    println!("{} + {} = {}", 1, 2, 3); // multiple variabel
    println!(); //print kosong

    //2. print! - print tanpa newline
    print!("Hitung: ");
    print!("1 ");
    print!("2 ");
    print!("3"); // newline baru ada disini
    //Output: Hitung: 1 2 3

    //3. eprintln! - Print ke stderr
    println!();
    println!();
    println!("Ini ke stdout - output normal");
    eprintln!("Ini ke stderr - untuk error/debug");
    //Penting karena, stdout dan stderr diarahkan ke tempat berbeda seperti (logfile, monitoring)

    //4. Format Specifiers {} - Display (Human Readable)
    println!();

    let name = "Jony";
    let amount: f64 = 150000.50;
    let count: i32 = 42;

    println!("Name: {}", name);
    println!("Amount: {}", amount);
    println!("Count: {}", count);

    // Named argument - lebih readable
    println!("Halo {name}, saldo kamu {amount }");


}