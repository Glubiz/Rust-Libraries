use crate::traits::Join;

impl Join for Vec<String> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, string) in self.iter().enumerate() {
            s.push_str(string);
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }

    

}

impl Join for Vec<&str> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, string) in self.iter().enumerate() {
            s.push_str(string);
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<char> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, c) in self.iter().enumerate() {
            s.push(*c);
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<u8> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<i8> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<u16> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<i16> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<u32> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<i32> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<u64> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<i64> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<u128> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<i128> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<usize> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<isize> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<f32> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<f64> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter().enumerate() {
            s.push_str(&b.to_string());
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

impl Join for Vec<bool> {
    fn join(&self, delimiter: char) -> String {
        let mut s = String::new();
        for (i, b) in self.iter()
            .map(|b| if *b { "true" } else { "false" })
            .enumerate()
        {
            s.push_str(b);
            if i < self.len() - 1 {
                s.push(delimiter);
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_string() {
        let v = vec!["a", "b", "c"];
        assert_eq!(v.join(','), "a,b,c");
    }

    #[test]
    fn test_join_char() {
        let v = vec!['a', 'b', 'c'];
        assert_eq!(v.join(','), "a,b,c");
    }

    #[test]
    fn test_join_u8() {
        let v = vec![1u8, 2u8, 3u8];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_i8() {
        let v = vec![1i8, 2i8, 3i8];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_u16() {
        let v = vec![1u16, 2u16, 3u16];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_i16() {
        let v = vec![1i16, 2i16, 3i16];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_u32() {
        let v = vec![1u32, 2u32, 3u32];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_i32() {
        let v = vec![1i32, 2i32, 3i32];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_u64() {
        let v = vec![1u64, 2u64, 3u64];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_i64() {
        let v = vec![1i64, 2i64, 3i64];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_u128() {
        let v = vec![1u128, 2u128, 3u128];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_i128() {
        let v = vec![1i128, 2i128, 3i128];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_usize() {
        let v = vec![1usize, 2usize, 3usize];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_isize() {
        let v = vec![1isize, 2isize, 3isize];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_f32() {
        let v = vec![1f32, 2f32, 3f32];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_f64() {
        let v = vec![1f64, 2f64, 3f64];
        assert_eq!(v.join(','), "1,2,3");
    }

    #[test]
    fn test_join_bool() {
        let v = vec![true, false, true];
        assert_eq!(v.join(','), "true,false,true");
    }
}

