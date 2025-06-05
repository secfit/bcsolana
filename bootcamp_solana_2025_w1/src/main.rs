mod twosum;

fn main() {
    fizz_buzz();

    println!("{:?}", twosum::two_sum(vec![2, 7, 11, 15], 9));  // [0, 1]
    println!("{:?}", twosum::two_sum(vec![3, 2, 4], 6));       // [1, 2]
    println!("{:?}", twosum::two_sum(vec![3, 3], 6));          // [0, 1]
}

/// fizz_buzz function loops from 1 to 300 (inclusive),
/// and prints "fizz", "buzz", or "fizz buzz" depending on the number.
/// It also counts how many times "fizz buzz" is printed.
fn fizz_buzz() {
    // Counter for number of times "fizz buzz" occurs
    let mut fizz_buzz_count = 0;

    // Loop from 1 to 300 (inclusive)
    for i in 1..=300 {
        if i % 3 == 0 && i % 5 == 0 {
            // Divisible by both 3 and 5
            println!("{}: fizz buzz", i);
            fizz_buzz_count += 1;
        } else if i % 3 == 0 {
            // Only divisible by 3
            println!("{}: fizz", i);
        } else if i % 5 == 0 {
            // Only divisible by 5
            println!("{}: buzz", i);
        }
//        } else {
            // Not divisible by 3 or 5, print the number itself
//            println!("{}", i);
//        }
    }

    // After the loop, print how many times "fizz buzz" was printed
    println!("'fizz buzz' occurred {} times.", fizz_buzz_count);
}
