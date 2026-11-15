// Simple Rust program: basic calculator + number utilities
// Demonstrates functions, structs, enums, and control flow.

use std::fmt;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        };
        write!(f, "{}", symbol)
    }
}

struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator { history: Vec::new() }
    }

    fn calculate(&mut self, a: f64, b: f64, op: Operation) -> Option<f64> {
        let result = match op {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => {
                if b == 0.0 {
                    println!("Error: division by zero");
                    return None;
                }
                a / b
            }
        };

        let entry = format!("{} {} {} = {}", a, op, b, result);
        self.history.push(entry);
        Some(result)
    }

    fn print_history(&self) {
        println!("\n--- Calculation History ---");
        for (i, entry) in self.history.iter().enumerate() {
            println!("{}. {}", i + 1, entry);
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();
    let (mut a, mut b): (u64, u64) = (0, 1);
    for _ in 0..n {
        sequence.push(a);
        let next = a + b;
        a = b;
        b = next;
    }
    sequence
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}

fn main() {
    println!("=== Rust Mini Toolkit ===\n");

    // 1. Calculator demo
    let mut calc = Calculator::new();
    calc.calculate(10.0, 5.0, Operation::Add);
    calc.calculate(10.0, 5.0, Operation::Subtract);
    calc.calculate(10.0, 5.0, Operation::Multiply);
    calc.calculate(10.0, 5.0, Operation::Divide);
    calc.calculate(10.0, 0.0, Operation::Divide);
    calc.print_history();

    // 2. Prime number check
    println!("\n--- Prime Numbers up to 30 ---");
    let primes: Vec<u32> = (2..=30).filter(|&n| is_prime(n)).collect();
    println!("{:?}", primes);

    // 3. Fibonacci sequence
    println!("\n--- First 10 Fibonacci Numbers ---");
    let fib = fibonacci(10);
    println!("{:?}", fib);

    // 4. String utilities
    println!("\n--- String Utilities ---");
    let sample = "Hello, Rust World!";
    println!("Original: {}", sample);
    println!("Reversed: {}", reverse_string(sample));
    println!("Vowel count: {}", count_vowels(sample));

    // 5. Simple loop + condition logic
    println!("\n--- FizzBuzz (1 to 20) ---");
    for i in 1..=20 {
        if i % 15 == 0 {
            println!("{}: FizzBuzz", i);
        } else if i % 3 == 0 {
            println!("{}: Fizz", i);
        } else if i % 5 == 0 {
            println!("{}: Buzz", i);
        } else {
            println!("{}: {}", i, i);
        }
    }

    println!("\n=== Done ===");
}
