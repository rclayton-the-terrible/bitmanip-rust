mod decimal_places_vec;
mod count_digits_in_integer;
mod decimal_to_binary;
mod common;
mod bits_set;

fn main() {
    println!(
        "What is the binary representation of a number, when n = 245? Answer={:?}",
        decimal_to_binary::bitshift_method(245)
    );

    println!(
        "Count the number of bits present in number, n = 10.  Answer={}",
        decimal_to_binary::bitshift_method(10).len()
    );

    println!(
        "Find the value of x & y. x={x}, y={y} Answer={result}",
        x=2,
        y=10,
        result = 2 & 10
    );

    println!(
        "Find the value of ( x & y) & z = ?. x={x}, y={y}, z={z} Answer={result}",
        x=12,
        y=10,
        z=32,
        result = (12 & 10) & 32
    );

    println!(
        "What’s the output of this expression (( 5 & 1) == 1 )? Answer={}",
        (5 & 1) == 1
    )
}
