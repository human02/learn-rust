fn main() {
    /* Array (in Rust) :

    - Collection of elements with the same data type
    - Fixed length
    - Stored in contiguous memory location
    - Stored in order
    - has zero (0) indexing like many other languages
    */

    let mut letters = ['a', 'b', 'c'];
    let mut first_lttr = letters[0];
    println!("first letter is --> {}", first_lttr);
    letters[0] = 'z';
    first_lttr = letters[0];
    println!("changed the first letter to --> {}", first_lttr);

    // Rust allows array decalaration w/o initial values.
    let nums: [i32; 7]; // here nums is of i32 type and has fixed length of 7
    nums = [1, 1, 1, 1, 1, 1, 1];
    let more_nums: [i32; 100]; // can't initialise this array the same way.
    more_nums = [1; 100]; // using repeat expression to initialise now.

    /*
     Index variable will be of 'usize' datatype and its based on no. of bytes needed to reference memory.
     Compiling for 32 bit processor - usize is 4 bytes
     Compiling for 64 bit processor - usize is 8 bytes
    */
    let index: usize = more_nums.len();
    println!(
        "Last number in more_nums array is {}\n",
        more_nums[index - 1]
    );
}
