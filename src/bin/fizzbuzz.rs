use exer::fizzbuzz;

pub fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}

fn main() {
    print_fizzbuzz_to(20)
}
