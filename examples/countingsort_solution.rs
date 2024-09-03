// Bubble sort

use std::fmt;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the number of items and max value.
    let num_items = get_i32("# Items: ");
    let max = get_i32("Max: ");

    // Make and display the unsorted vector.
    let mut customers = make_random_vec(num_items, max);
    print_vec(&customers, 20);
    println!();

    // Sort and display the result.
    customers = counting_sort(&mut customers[..], max);
    print_vec(&customers, 20);

    // Verify that it's sorted.
    check_sorted(&customers);
}

//*************************
//*** The Customer struct *
//*************************
struct Customer {
    id: String,
    num_purchases: i32,
}
impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

// Make a vector containing pseudorandom
// customers with num_purchases in [0, max).
// The ith customer has id C<i> as in C103.
fn make_random_vec(num_items: i32, max: i32) -> Vec<Customer> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut customers: Vec<Customer> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        let id = format!("C{i}");
        let num_purchases = prng.next_i32(0, max);
        let customer = Customer {
            id: id,
            num_purchases: num_purchases,
        };
        customers.push(customer);
    }
    return customers;
}

// Print at most num_items items.
fn print_vec(vec: &Vec<Customer>, num_items: i32) {
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

// Sort a vector containing Customers with num_purchases in [0, max).
fn counting_sort(slice: &mut [Customer], max: i32) -> Vec<Customer> {
    let num_items = slice.len();

    // Make a vector to hold counts. Initialize with 0s.
    let mut counts = vec![0i32; max as usize];

    // Count the values.
    for i in 0..num_items {
        counts[slice[i].num_purchases as usize] += 1;
    }

    // Convert counts into counts of values <=.
    for i in 1usize..max as usize {
        counts[i] += counts[i - 1];
    }

    // Count out the values.
    let mut result: Vec<Customer> = Vec::with_capacity(num_items as usize);
    unsafe {
        result.set_len(num_items);
    }
    for i in (0..num_items).rev() {
        // Copy item i into position.
        let num = slice[i].num_purchases as usize;
        result[(counts[num] - 1) as usize] = Customer {
            id: slice[i].id.clone(),
            num_purchases: slice[i].num_purchases,
        };
        counts[num] -= 1;
    }

    return result;
}

// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<Customer>) {
    for i in 1usize..vec.len() {
        if vec[i - 1].num_purchases > vec[i].num_purchases {
            println!("The vector is NOT sorted!");
            return;
        }
    }
    println!("The vector is sorted!");
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
