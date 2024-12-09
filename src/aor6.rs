// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    match s1.trim().chars().count().cmp(&s2.trim().chars().count()) {
        std::cmp::Ordering::Greater => Some(s1),
        std::cmp::Ordering::Less => Some(s2),
        std::cmp::Ordering::Equal => None
    }
}
