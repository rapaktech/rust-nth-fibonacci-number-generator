use std::io;

fn main() {
    loop {
        let mut counter: u64 = 1;

        let mut new_counter: u64 = 1;

        println!("Input the position you're looking for:");

        let mut nth = String::new();

        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line!");

        let nth: u64 = match nth.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please input a valid number!");
                continue
            },
        };

        let mut i = 2;

        while i < nth && nth > 2 {
            let old_counter = new_counter.clone();

            new_counter = counter + new_counter;

            counter = old_counter;

            i = i + 1;
        }

        println!("the Fibonaccci number at position {nth} is {new_counter}");
    }
}
