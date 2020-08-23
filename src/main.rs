macro_rules! print_hello {
    () => {
        println!("hi");
    };
}

/// This is the EntryPoint
fn main() {
    print_hello!();

    // 31 is now type i64
    println!("{} days", 31i64);
    println!("also {days} days", days=35i64);

}
