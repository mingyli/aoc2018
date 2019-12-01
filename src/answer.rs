use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Answer {
    I(i32),
    U(u32),
    US(usize),
    S(String),
}

impl Answer {
    fn int_val(&self) -> Option<i32> {
        match *self {
            Answer::I(i) => Some(i),
            _ => None,
        }
    }

    fn uint_val(&self) -> Option<u32> {
        match *self {
            Answer::U(u) => Some(u),
            _ => None,
        }
    }

    fn usize_val(&self) -> Option<usize> {
        match *self {
            Answer::US(us) => Some(us),
            _ => None,
        }
    }

    fn string_val(&self) -> Option<&String> {
        match self {
            Answer::S(s) => Some(&s),
            _ => None,
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::I(i) => write!(f, "{}", i),
            Answer::U(u) => write!(f, "{}", u),
            Answer::US(us) => write!(f, "{}", us),
            Answer::S(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq<i32> for Answer {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Answer::I(_) => self.int_val() == Some(*other),
            Answer::U(_) => self.uint_val() == Some(*other as u32),
            Answer::US(_) => self.usize_val() == Some(*other as usize),
            _ => false,
        }
    }
}
impl PartialEq<Answer> for i32 {
    fn eq(&self, other: &Answer) -> bool {
        other == self
    }
}

impl PartialEq<u32> for Answer {
    fn eq(&self, other: &u32) -> bool {
        self.uint_val() == Some(*other)
    }
}
impl PartialEq<Answer> for u32 {
    fn eq(&self, other: &Answer) -> bool {
        other == self
    }
}

impl PartialEq<usize> for Answer {
    fn eq(&self, other: &usize) -> bool {
        self.usize_val() == Some(*other)
    }
}
impl PartialEq<Answer> for usize {
    fn eq(&self, other: &Answer) -> bool {
        other == self
    }
}

impl PartialEq<String> for Answer {
    fn eq(&self, other: &String) -> bool {
        self.string_val() == Some(other)
    }
}
impl PartialEq<Answer> for String {
    fn eq(&self, other: &Answer) -> bool {
        other == self
    }
}

impl PartialEq<&str> for Answer {
    fn eq(&self, other: &&str) -> bool {
        self.string_val() == Some(&other.to_string())
    }
}
impl PartialEq<Answer> for &str {
    fn eq(&self, other: &Answer) -> bool {
        other == self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        assert_eq!(Answer::I(5), Answer::I(5));
        assert_ne!(Answer::I(6), Answer::I(5));
        assert_eq!(Answer::I(5), 5);
        assert_eq!(5, Answer::I(5));
        assert_ne!(Answer::I(5), 6);
        assert_ne!(5, Answer::I(6));
    }

    #[test]
    fn test_uint() {
        assert_eq!(Answer::U(5), Answer::U(5));
        assert_ne!(Answer::U(6), Answer::U(5));
        assert_eq!(Answer::U(5), 5);
        assert_eq!(5, Answer::U(5));
        assert_ne!(Answer::U(5), 6);
        assert_ne!(5, Answer::U(6));
    }

    #[test]
    fn test_usize() {
        assert_eq!(Answer::US(5), Answer::US(5));
        assert_ne!(Answer::US(6), Answer::US(5));
        assert_eq!(Answer::US(5), 5);
        assert_eq!(5, Answer::US(5));
        assert_ne!(Answer::US(5), 6);
        assert_ne!(5, Answer::US(6));
    }

    #[test]
    fn test_string() {
        let mut s = String::new();
        s.push('h');
        s.push('e');
        assert_eq!(Answer::S(s), Answer::S("he".to_string()));
        assert_ne!(Answer::S("as".to_string()), Answer::S("bs".to_string()));
        assert_eq!(Answer::S("sg".to_string()), "sg".to_string());
        assert_eq!("sg".to_string(), Answer::S("sg".to_string()));
        assert_ne!(Answer::S("jk".to_string()), "j".to_string());
        assert_ne!("jk".to_string(), Answer::S("j".to_string()));
        assert_eq!("jk", Answer::S("jk".to_string()));
        assert_eq!(Answer::S("jk".to_string()), "jk");
        assert_ne!("j", Answer::S("jk".to_string()));
        assert_ne!(Answer::S("j".to_string()), "jk");
    }

    #[test]
    fn test_mix() {
        assert_ne!(Answer::U(5), Answer::I(5));
        assert_ne!(Answer::US(5), Answer::U(5));
        assert_ne!(Answer::I(5), Answer::S("f".to_string()));
        assert_ne!(Answer::US(7), Answer::S("g".to_string()));
    }
}
