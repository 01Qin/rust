fn main()
{
    let name = "John"; // &str, immutable fixed length string and it is also called string slice.
    let name = String::from("John"); // mutable UTF-8 encoded string type. from is an associated function of the String type.

    let mut message = String::from("Hello"); // mutable variable
    hello.push('w';) // append a char to a String using the push method
    hello.push_str("orld!"); // append a &str to a String using the push_str method

    let s1 = String::from("Hello, world!");
    let s2 = s.replace("world", "Rust"); // s2 = "Hello, Rust!"
    // replace function replaces all non-overlapping occurrences of a specified pattern with another string.
    
}