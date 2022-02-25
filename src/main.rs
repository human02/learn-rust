fn main() {
    let x = 10;
    println!("\nHello - World\nx = {}", x);
    let mut y = 1;
    println!("\ncurrently y = {}", y);
    y = y + x;
    println!("\nNow y is mutable and its value is now = {}", y);
    let z = '\u{1F595}'; // using unicode value s char is stored in 4 byte instead of 1 byte in Rust
    println!(" Hello Peeps its time for you all to {}  ", z);

    let a1 = 13;
    let a2 = 2.3;
    let a3: f32 = 120.0;

    let avg = (a1 as f32 + a2 + a3) / 3 as f32; // Type casting using "as". arithmetic operations cant be done on different datatype variables.
    println!("average of a1,a2,a3 = {}", avg);
}
