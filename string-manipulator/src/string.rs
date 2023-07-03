use crate::traits::{Compression, ConvertCase, Padding, Replace, Search, Split, Substring, Trim};

impl Trim for String {
    fn trim(&self) -> String {
        self.clone().trim_start().trim_end().to_string()
    }
}

impl ConvertCase for String {
    fn to_camel_case(&self) -> String {
        let mut s = String::new();
        let mut capitalize = false;
        for c in self.chars() {
            if c == '_' || c == '-' || c == ' ' {
                capitalize = true;
            } else if capitalize {
                s.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                s.push(c);
            }
        }
        s
    }

    fn to_snake_case(&self) -> String {
        let mut s = String::new();
        for c in self.chars() {
            if c.is_ascii_uppercase() {
                s.push('_');
                s.push(c.to_ascii_lowercase());
            } else if c == '-' || c == ' ' {
                s.push('_');
            } else {
                s.push(c);
            }
        }
        s
    }

    fn to_kebab_case(&self) -> String {
        let mut s = String::new();
        for c in self.chars() {
            if c.is_ascii_uppercase() {
                s.push('-');
                s.push(c.to_ascii_lowercase());
            } else if c == '_' || c == ' ' {
                s.push('-');
            } else {
                s.push(c);
            }
        }
        s
    }

    fn to_upper(&self) -> String {
        self.to_ascii_uppercase()
    }

    fn to_lower(&self) -> String {
        self.to_ascii_lowercase()
    }
}

impl Padding for String {
    fn pad_left(&self, length: usize, pad: char) -> String {
        let mut s = String::new();
        for _ in 0..length - self.len() {
            s.push(pad);
        }
        s.push_str(self);
        s
    }

    fn pad_right(&self, length: usize, pad: char) -> String {
        let mut s = String::new();
        s.push_str(self);
        for _ in 0..length - self.len() {
            s.push(pad);
        }
        s
    }

    fn pad(&self, n: usize, c: char) -> Self {
        let mut s = String::new();
        for _ in 0..n {
            s.push(c);
        }
        s.push_str(self);
        for _ in 0..n {
            s.push(c);
        }
        s
    }
}

impl Split for String {
    fn split(&self, delimiter: char) -> Vec<String> {
        let mut v = Vec::new();
        let mut s = String::new();
        for c in self.chars() {
            if c == delimiter {
                v.push(s);
                s = String::new();
            } else {
                s.push(c);
            }
        }
        v.push(s);
        v
    }
}

impl Search for String {
    fn find_first_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().enumerate() {
            if s.contains(c) {
                return Some(i);
            }
        }
        None
    }

    fn find_last_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().rev().enumerate() {
            if s.contains(c) {
                return Some(self.len() - i - 1);
            }
        }
        None
    }

    fn find_first_not_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().enumerate() {
            if !s.contains(c) {
                return Some(i);
            }
        }
        None
    }

    fn find_last_not_of(&self, s: &str) -> Option<usize> {
        for (i, c) in self.chars().rev().enumerate() {
            if !s.contains(c) {
                return Some(self.len() - i - 1);
            }
        }
        None
    }

    fn matches_count(&self, s: &str) -> usize {
        self.matches(s).count()
    }

    fn matches_indices(&self, s: &str) -> Vec<usize> {
        let mut v = Vec::new();
        let mut pos = 0;
        while let Some(start) = self[pos..].find(s) {
            v.push(pos + start);
            pos += start + s.len();
        }
        v
    }

    fn matches_indices_count(&self, s: &str) -> usize {
        let mut count = 0;
        let mut pos = 0;
        while let Some(start) = self[pos..].find(s) {
            count += 1;
            pos += start + s.len();
        }
        count
    }

    fn contains_any(&self, s: &str) -> bool {
        for c in self.chars() {
            if s.contains(c) {
                return true;
            }
        }
        false
    }

    fn contains_all(&self, s: &str) -> bool {
        for c in s.chars() {
            if !self.contains(c) {
                return false;
            }
        }
        true
    }

    fn contains_none(&self, s: &str) -> bool {
        for c in self.chars() {
            if s.contains(c) {
                return false;
            }
        }
        true
    }
}

impl Replace for String {
    fn replace_first(&self, from: &str, to: &str) -> String {
        let mut s = String::new();
        if let Some(start) = self.find(from) {
            s.push_str(&self[..start]);
            s.push_str(to);
            s.push_str(&self[start + from.len()..]);
        } else {
            s.push_str(self);
        }
        s
    }

    fn replace_last(&self, from: &str, to: &str) -> String {
        let mut s = String::new();
        if let Some(start) = self.rfind(from) {
            s.push_str(&self[..start]);
            s.push_str(to);
            s.push_str(&self[start + from.len()..]);
        } else {
            s.push_str(self);
        }
        s
    }

    fn replace_all(&self, from: &str, to: &str) -> String {
        let mut s = String::new();
        let mut pos = 0;
        while let Some(start) = self[pos..].find(from) {
            s.push_str(&self[pos..pos + start]);
            s.push_str(to);
            pos += start + from.len();
        }
        s.push_str(&self[pos..]);
        s
    }
}

impl Substring for String {
    fn substring(&self, start: usize, end: usize) -> String {
        let mut s = String::new();
        for c in self.chars().skip(start).take(end - start) {
            s.push(c);
        }
        s
    }

    fn substring_from(&self, start: usize) -> String {
        let mut s = String::new();
        for c in self.chars().skip(start) {
            s.push(c);
        }
        s
    }

    fn substring_to(&self, end: usize) -> String {
        let mut s = String::new();
        for c in self.chars().take(end) {
            s.push(c);
        }
        s
    }
}

impl Compression for String {
    fn compress(&self) -> String {
        let mut compressed = String::new();
        let mut chars = self.chars().peekable();

        while let Some(current_char) = chars.next() {
            let mut count = 1;
            while chars.peek() == Some(&current_char) {
                chars.next();
                count += 1;
            }
            compressed.push_str(&format!("{}{}", current_char, count));
        }

        compressed
    }

    fn decompress(&self) -> String {
        let mut decompressed = String::new();
        let mut chars = self.chars().peekable();
    
        while let Some(current_char) = chars.next() {
            let count: String = chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
            let count: usize = count.parse().unwrap_or(1);
            decompressed.push_str(&current_char.to_string().repeat(count));
        }
    
        decompressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        let s = String::from("  hello world  ");
        assert_eq!(s.trim(), "hello world");
    }

    #[test]
    fn test_to_camel_case() {
        let s = String::from("hello_world");
        assert_eq!(s.to_camel_case(), "helloWorld");
    }

    #[test]
    fn test_to_camel_case_space() {
        let s = String::from("hello world");
        assert_eq!(s.to_camel_case(), "helloWorld");
    }

    #[test]
    fn test_to_snake_case() {
        let s = String::from("helloWorld");
        assert_eq!(s.to_snake_case(), "hello_world");
    }

    #[test]
    fn test_to_snake_case_space() {
        let s = String::from("hello world");
        assert_eq!(s.to_snake_case(), "hello_world");
    }

    #[test]
    fn test_to_kebab_case() {
        let s = String::from("helloWorld");
        assert_eq!(s.to_kebab_case(), "hello-world");
    }

    #[test]
    fn test_to_upper() {
        let s = String::from("hello world");
        assert_eq!(s.to_upper(), "HELLO WORLD");
    }

    #[test]
    fn test_to_lower() {
        let s = String::from("HELLO WORLD");
        assert_eq!(s.to_lower(), "hello world");
    }

    #[test]
    fn test_pad_left() {
        let s = String::from("hello");
        assert_eq!(s.pad_left(10, ' '), "     hello");
    }

    #[test]
    fn test_pad_right() {
        let s = String::from("hello");
        assert_eq!(s.pad_right(10, ' '), "hello     ");
    }

    #[test]
    fn test_pad() {
        let s = String::from("hello");
        assert_eq!(s.pad(2, ' '), "  hello  ");
    }

    #[test]
    fn test_split() {
        let s = String::from("hello world");
        assert_eq!(s.split(' '), vec!["hello", "world"]);
    }

    #[test]
    fn test_find_first_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_first_of("o"), Some(4));
    }

    #[test]
    fn test_find_last_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_last_of("o"), Some(7));
    }

    #[test]
    fn test_find_first_not_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_first_not_of("hello"), Some(5));
    }

    #[test]
    fn test_find_last_not_of() {
        let s = String::from("hello world");
        assert_eq!(s.find_last_not_of("world"), Some(5));
    }

    #[test]
    fn test_matches_count() {
        let s = String::from("hello world");
        assert_eq!(s.matches_count("hello"), 1);
    }

    #[test]
    fn test_matches_indices() {
        let s = String::from("hello world");
        assert_eq!(s.matches_indices("hello"), vec![0]);
    }

    #[test]
    fn test_matches_indices_count() {
        let s = String::from("hello world");
        assert_eq!(s.matches_indices_count("hello"), 1);
    }

    #[test]
    fn test_contains_any() {
        let s = String::from("hello world");
        assert!(s.contains_any("hello"));
    }

    #[test]
    fn test_contains_all() {
        let s = String::from("hello world");
        assert!(s.contains_all("hello"));
    }

    #[test]
    fn test_contains_none() {
        let s = String::from("hello world");
        assert!(!s.contains_none("hello"));
    }

    #[test]
    fn test_replace_first() {
        let s = String::from("hello world");
        assert_eq!(s.replace_first("hello", "goodbye"), "goodbye world");
    }

    #[test]
    fn test_replace_last() {
        let s = String::from("hello world");
        assert_eq!(s.replace_last("world", "universe"), "hello universe");
    }

    #[test]
    fn test_replace_all() {
        let s = String::from("hello world");
        assert_eq!(s.replace_all("l", "L"), "heLLo worLd");
    }

    #[test]
    fn test_substring() {
        let s = String::from("hello world");
        assert_eq!(s.substring(0, 5), "hello");
    }

    #[test]
    fn test_substring_from() {
        let s = String::from("hello world");
        assert_eq!(s.substring_from(6), "world");
    }

    #[test]
    fn test_substring_to() {
        let s = String::from("hello world");
        assert_eq!(s.substring_to(5), "hello");
    }

    #[test]
    fn test_compress() {
        assert_eq!(String::from("aaabbbccc").compress(), "a3b3c3");
        assert_eq!(
            String::from("Hello, world!").compress(),
            "H1e1l2o1,1 1w1o1r1l1d1!1"
        );
    }

    #[test]
    fn test_decompress() {
        assert_eq!(String::from("a3b3c3").decompress(), "aaabbbccc");
        assert_eq!(
            String::from("H1e1l2o1,1 1w1o1r1l1d1!1").decompress(),
            "Hello, world!"
        );
    }

    #[test]
    fn test_compress_decompress() {
        let original = String::from("Hello, world!");
        let compressed = original.compress();
        let decompressed = compressed.decompress();
        assert_eq!(decompressed, original);
    }
}
