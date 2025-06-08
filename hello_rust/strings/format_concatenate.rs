fn main()
{
    let string1 = String :: from("Hello, ");
    let string2 = "world!";

    // string is not moved and can still be used
    let concatenated = format!("{}{}", string1, string2);

    println!("{}", concatenated);
    println!("{}", string1); // string1 can still be used
}