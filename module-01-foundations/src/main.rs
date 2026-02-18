fn main() {
    // modules 01.01
    println!("Modules 01.01 - Basic Rust");
    println!("Hello, World!");

    //modules 01.02
    println!();
    // Basic Macro
    println!("Modules 01.02 - Basic Macro Rust");
    // 1. println! - print new line
    println!("1. println! - print new line");
    println!("Hello, Rust"); // normal teks
    println!("Nilai: {}", 42); // satu variabel
    println!("{} + {} = {}", 1, 2, 3); // multiple variabel
    println!(); //print kosong

    //2. print! - print tanpa newline
    println!("2. print! - print tanpa newline");
    print!("Hitung: ");
    print!("1 ");
    print!("2 ");
    print!("3"); // newline baru ada disini
    //Output: Hitung: 1 2 3

    //3. eprintln! - Print ke stderr
    println!();
    println!();
    println!("3. eprintln! - Print ke stderr");
    println!("Ini ke stdout - output normal");
    eprintln!("Ini ke stderr - untuk error/debug");
    //Penting karena, stdout dan stderr diarahkan ke tempat berbeda seperti (logfile, monitoring)

    //4. Format Specifiers {} - Display (Human Readable)
    println!();
    println!("4. Format Specifiers - Display (Human Readable)");

    let name = "Jony";
    let amount: f64 = 150000.50;
    let count: i32 = 42;

    println!("Name: {}", name);
    println!("Amount: {}", amount);
    println!("Count: {}", count);

    // Named argument - lebih readable
    println!("Halo {name}, saldo kamu {amount }");

    //5. Format Debug {:?} - Developer Readable
    println!();
    println!("5. Format Debug - Developer Readable");
    let numbers = vec![1,2,3,4,5];
    let tuple = (10, "hello", true);

    //{} tidak bisa menggunakan tupple
    println!("{:?}", numbers);
    println!("{:?}", tuple);

    //6. Format Debug Multi Line {:#?} - Debug Developer Readable with multi line
    println!();
    println!("6. Format Debug Multi Line - Debug Developer Readable with multi line");
    // Get data numbers from format debug number 5 line 42
    println!("{:#?}", numbers);

    //7. Format Angka
    println!();
    println!("7. Format Angka");
    let pi = 3.14159265;
    let amount = 150000.0;
    let percent = 0.875;

    //Decimal Places
    println!("{:.2}", pi);
    println!("{:.0}", pi);

    //Width and alignment
    println!("{:>10}", "kanan");    // right align
    println!("{:<10}", "kiri");     // left align
    println!("{:^10}", "tengah");   // center align

    //Fill dengan karakter
    println!("{:0>5}", 42);         // 00042 (zero padding)
    println!("{:*>10}", "rust");    // ******rust

    //Separator ribuan --- menggunakan underscored (_) untuk readability code
    let big = 1_000_000;
    println!("{}", big);        //1000000 (tidak ada separator)

    //7. format! - Digunakan untuk response API atau message detail error code
    println!();
    println!("7. format! - Digunakan untuk response API atau message detail error code");
    // Get data name from format specifier number 4 line 33 and data amount from format angka line 68

    //Create string data dan print
    let message = format!("Halo {}, transaksi Rp {} berhasil", name, amount);

    println!("{}", message);

    //Contoh usecase: build error message
    let error_msg = format!("Transaction {} failed: insufficient balance", "TRX-001");
    eprintln!("{}", error_msg);

    //8. Vector
    println!();
    println!("8. Vector");

    //Tanpa menggunakan macro - verbose
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    println!("Tanpa menggunakan macro: {:?}", v1);

    //Dengan macro lebih simple
    let v2 = vec![1,2,3];
    println!("Menggunakan macro: {:?}", v2);
}