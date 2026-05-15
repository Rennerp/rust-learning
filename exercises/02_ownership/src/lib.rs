#![allow(unused_variables)]

pub fn first_word(text: &str) -> &str {
    // Return a slice borrowed from text, or "" when there is no word.
    todo!("return the first whitespace-separated word")
}

pub fn normalize_name(name: &mut String) {
    // Build the normalized value, then assign it back into the borrowed String.
    todo!("trim the name and lowercase it in place")
}

pub fn join_words(words: &[String]) -> String {
    // Borrow the slice and create a new joined String.
    todo!("join borrowed words with a single space")
}

pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    // The returned reference must be one of the input references.
    todo!("return the longer string slice; return left when tied")
}

pub fn remove_empty(values: &mut Vec<String>) {
    // Mutate the existing vector instead of returning a new one.
    todo!("remove empty or whitespace-only strings from the vector")
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
