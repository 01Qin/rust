fn main()
{
    let string1 = String::from("Hello, ");
    let string2 = "world!";

    //string1 is moved here and can no longer be used
    let concatenated = string1 + string2;

    println!("{}", concatenated);
}