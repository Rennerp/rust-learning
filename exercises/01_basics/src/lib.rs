#![allow(unused_variables)]

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // Formula: (F - 32) * 5 / 9.
    todo!("convert Fahrenheit to Celsius")
}

pub fn factorial(number: u64) -> u64 {
    // Return 1 for 0, then multiply the range from 1 through number.
    todo!("return the factorial of number")
}

pub fn fizzbuzz(number: u32) -> String {
    // Check divisibility by 15 before checking 3 or 5.
    todo!("return Fizz, Buzz, FizzBuzz, or the number as text")
}

pub fn count_vowels(text: &str) -> usize {
    // Iterate over chars so the code reads text characters, not bytes.
    todo!("count a, e, i, o, u in lowercase and uppercase text")
}

pub fn sum_even_numbers(numbers: &[i32]) -> i32 {
    // Read from the borrowed slice; do not take ownership of it.
    todo!("sum only even numbers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn calculates_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn formats_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn counts_vowels() {
        assert_eq!(count_vowels("Rust"), 1);
        assert_eq!(count_vowels("Education"), 5);
        assert_eq!(count_vowels("rhythms"), 0);
    }

    #[test]
    fn sums_even_numbers() {
        assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12);
        assert_eq!(sum_even_numbers(&[-2, -1, 0, 1, 2]), 0);
    }
}
