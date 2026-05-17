#![allow(unused_variables)]

pub fn first_word(text: &str) -> &str {
    // Return a slice borrowed from text, or "" when there is no word.
    let size: usize = text.len();
    for (i, c) in text.char_indices() {
        if c.is_whitespace() {
            return &text[0..i];
        }
    }

    &text[0..size]
}

pub fn normalize_name(name: &mut String) {
    // Change the existing String instead of returning a new one.
    // Remove leading and trailing whitespace, lowercase the text,
    // then write the normalized result back through the mutable borrow.
    // Example: "  Renner Poveda  " becomes "renner poveda".

    /*
        Explanation: trim() does not modify the original String;
        it only returns a borrowed view without leading or trailing whitespace.
        To update the actual value, create the normalized text with to_lowercase() and
        assign that new String back through the mutable reference.
     */
    *name = name.trim().to_lowercase();
}

pub fn join_words(words: &[String]) -> String {
    // Read from the borrowed slice without taking ownership of any String.
    // Build and return one new String with a single space between each word.
    // Example: ["ownership", "borrowing", "lifetimes"] becomes
    // "ownership borrowing lifetimes".
    words.join(" ")
}

pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    // Return one of the borrowed inputs instead of creating new text.
    // Compare the lengths and return the longer slice.
    // If both have the same length, return left.
    //todo!("return the longer borrowed string slice, using left as the tie-breaker")

    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

pub fn remove_empty(values: &mut Vec<String>) {
    // Mutate the existing vector through the mutable borrow.
    // Keep only entries that contain non-whitespace text.
    // Remove both "" and strings like "  ".
    values.retain(|value| !value.trim().is_empty() ) ;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_word() {
        assert_eq!(first_word("hello rust"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn normalizes_name_in_place() {
        let mut name = String::from("  Renner Poveda  ");
        normalize_name(&mut name);
        assert_eq!(name, "renner poveda");
    }

    #[test]
    fn joins_without_taking_ownership() {
        let words = vec![
            String::from("ownership"),
            String::from("borrowing"),
            String::from("lifetimes"),
        ];

        assert_eq!(join_words(&words), "ownership borrowing lifetimes");
        assert_eq!(words.len(), 3);
    }

    #[test]
    fn returns_longest_slice() {
        assert_eq!(longest("short", "longer"), "longer");
        assert_eq!(longest("same", "ties"), "same");
    }

    #[test]
    fn removes_empty_strings_in_place() {
        let mut values = vec![
            String::from("rust"),
            String::from(""),
            String::from("  "),
            String::from("cargo"),
        ];

        remove_empty(&mut values);
        assert_eq!(values, vec![String::from("rust"), String::from("cargo")]);
    }
}
