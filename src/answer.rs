use std::fmt;

pub enum Answer {
    I(i32),
    U(u32),
    US(usize),
    S(String),
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
