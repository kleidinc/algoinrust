// Linear search

use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the number of items and max value.
    let num_items = get_i32("# Items: ");
    let max = get_i32("Max: ");

    // Make and display the unsorted vector.
    let mut numbers = make_random_vec(num_items, max);
    print_vec(&numbers, 100);
    println!();

    loop {
        // Get the target as a string.
        let target = get_i32("Target (-1 to quit): ");
        if target < 0 {
            break;
        }

        let index: i32;
        let num_tests: i32;
        (index, num_tests) = linear_search(&numbers, target);
        if index < 0 {
            println!("Target {target} not found, {num_tests} tests");
        } else {
            let result = numbers[index as usize];
            println!("numbers[{index}] = {result}, {num_tests} tests");
        }
        println!();
    }
}

// Make a vector of random i32 values in the range [0 and max).
fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();
    prng.randomize();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}

// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

// Perform linear search.
// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn linear_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut num_tests = 0;
    for i in 0..vec.len() {
        num_tests += 1;
        if vec[i] == target {
            return (i as i32, num_tests);
        }
    }
    return (-1, num_tests);
}

// *****************
// *** Utilities ***
// *****************
// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>().expect("Error parsing integer");
}

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}
