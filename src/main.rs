mod decimal_places_vec;
mod count_digits_in_integer;
mod decimal_to_binary;
mod common;
mod bits_set;
mod odd_or_even;
mod power_of_two;
mod min_flips;
mod swap;
mod odd_occurring_element;
mod opposite_signs;
mod find_missing_number;

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
    );

    println!(
        "Find the value of x | y. x={x}, y={y} Answer={result}",
        x=2,
        y=10,
        result = 2 | 10
    );

    println!(
        "Find the value of ( x | y) | z. x={x}, y={y}, z={z} Answer={result}",
        x=12,
        y=10,
        z=32,
        result = (12 | 10) | 32
    );

    println!(
        "Find the output of the following left shift operators: X << 1, X << 2, where X = 5.  Answer={}, {}",
        5 << 1,
        5 << 2
    );

    println!(
        "Output of the following left shift operators: x << 3, x << 5, where x = 5.  Answer={}, {}",
        5 << 3,
        5 << 5
    );

    println!(
        "Find the output of the following right shift operators: X >> 1, X >> 2, where X = 5.  Answer={}, {}",
        5 >> 1,
        5 >> 2
    );

    println!(
        "Find the output of the following right shift operators: X >> 3, X >> 5, where X = 5.  Answer={}, {}",
        5 >> 3,
        5 >> 5
    );


}
