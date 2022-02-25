pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");
  
    // Basic Formatting
    println!("{} is from {}", "Aarav", "Chandra");
  
    // Positional Arguments
    println!(
      "{0} is from {1} and {0} likes to {2}",
      "Aarav", "Chandra", "code"
    );
  
    // Named Arguments
    println!(
      "{name} likes to play {activity}",
      name = "Aarav",
      activity = "Guitar"
    );
  
    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 1, 10, 100);
  
    // Placeholder for debug trait
    println!("{:?}", (121, true, "hello-world"));
  
    // Basic math
    println!("10 * 10 = {}", 10 * 10);
  }