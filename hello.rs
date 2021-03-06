
fn main() {
    println!("{}", square_sum(10));
}

fn fizzbuzz(n: usize) {
    for i in 0..n {
        if i % 15 == 0{
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn square_sum(n: isize) -> isize {
    (0..n).filter(|i| i%2 == 0)
        .map(|i| i*i)
        .sum()
}
