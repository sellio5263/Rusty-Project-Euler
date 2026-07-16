use std::collections::HashMap;
use std::iter;

pub fn triangle_number_iterator() -> impl Iterator<Item = u32> {
    let mut i: u32 = 0;
    iter::from_fn(move || {
        i += 1;
        Some(i * (i + 1) / 2)
    })
}

pub fn smarter_collatz_len(n: u64, known_lengths: &mut Vec<u32>) -> u32 {
    if n == 1 {
        return 1;
    }

    if n < 1_000_000 {
        let len = known_lengths[n as usize];
        if len != 0 {
            return len;
        }
    }

    let next = if n.is_multiple_of(2) {
        n / 2
    } else {
        3 * n + 1
    };

    let len = 1 + smarter_collatz_len(next, known_lengths);
    if n < 1_000_000 {
        known_lengths[n as usize] = len;
    }
    len
}

pub fn smart_collatz_len(n: u64, known_lengths: &mut HashMap<u64, u32>) -> u32 {
    if n == 1 {
        return 1;
    }

    if let Some(&len) = known_lengths.get(&n) {
        return len;
    }

    let next = if n.is_multiple_of(2) {
        n / 2
    } else {
        3 * n + 1
    };

    let len = 1 + smart_collatz_len(next, known_lengths);
    known_lengths.insert(n, len);
    len
}

pub struct Collatz {
    current: Option<u64>,
}

impl Collatz {
    pub fn new_collatz(start: u64) -> Self {
        Self {
            current: Some(start),
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.current {
            if n == 1 {
                self.current = None;
            } else if n.is_multiple_of(2) {
                self.current = Some(n / 2);
            } else {
                self.current = Some(3 * n + 1);
            }
            return Some(n);
        }
        None
    }
}
