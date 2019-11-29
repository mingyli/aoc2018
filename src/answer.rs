use std::fmt;

#[derive(Debug)]
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

    fn string_val(&self) -> Option<String> {
        match &*self {
            Answer::S(s) => Some(s.to_string()),
            _ => None,
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Answer::I(i) => write!(f, "{}", i),
            Answer::U(u) => write!(f, "{}", u),
            Answer::US(us) => write!(f, "{}", us),
            Answer::S(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Self) -> bool {
        match &*self {
            Answer::I(i) => *i == other.int_val().unwrap(),
            Answer::U(u) => *u == other.uint_val().unwrap(),
            Answer::US(us) => *us == other.usize_val().unwrap(),
            Answer::S(s) => *s == other.string_val().unwrap(),
        }
    }
}
