use std::fmt::Display;

#[derive(Clone)]
pub struct MegaUInt {
    digits: Vec<i8>,
}

pub fn mega_int_from_string(s: &str) -> MegaUInt {
    let digits = s
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    MegaUInt { digits }
}

impl MegaUInt {
    pub fn zero() -> MegaUInt {
        MegaUInt { digits: vec![0] }
    }

    pub fn one() -> MegaUInt {
        MegaUInt { digits: vec![1] }
    }

    pub fn digit_at(&self, index: usize) -> i8 {
        if index < self.digits.len() {
            self.digits[index]
        } else {
            0
        }
    }

    pub fn digits(&self) -> &[i8] {
        &self.digits
    }

    pub fn add(&self, other: &MegaUInt) -> MegaUInt {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = self.digits.len().max(other.digits.len());

        for i in 0..max_len {
            let sum = self.digit_at(i) + other.digit_at(i) + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        MegaUInt { digits: result }
    }

    pub fn subtract(&self, other: &MegaUInt) -> MegaUInt {
        if self.digits().len() < other.digits().len() {
            panic!("Cannot subtract a larger number from a smaller one");
        } else {
            // Unsure so try subtraction
            let mut result = Vec::new();
            let mut carry_penalty = 0;

            for i in 0..self.digits.len() {
                let sum = self.digit_at(i) - other.digit_at(i) - carry_penalty;
                result.push(sum % 10);
                carry_penalty = if sum < 0 { 1 } else { 0 };
            }

            if carry_penalty > 0 {
                panic!("Result would be negative");
            }

            MegaUInt { digits: result }
        }
    }

    pub fn multiply(&self, other: &MegaUInt) -> MegaUInt {
        let mut result = MegaUInt::zero();
        let mut partial_products: Vec<MegaUInt> = Vec::new();

        for (shift, &digit) in other.digits.iter().enumerate() {
            let partial_product = self.multiply_by_digit(digit);
            let shifted_partial_product = partial_product.shift(shift);
            partial_products.push(shifted_partial_product);
        }

        result = partial_products
            .into_iter()
            .fold(result, |acc, x| acc.add(&x));

        result
    }

    pub fn power(&self, exponent: u32) -> MegaUInt {
        let mut result = MegaUInt::one();
        for _ in 0..exponent {
            result = result.multiply(self);
        }
        result
    }

    fn shift(&self, shift: usize) -> MegaUInt {
        let mut shifted_digits = vec![0; shift];
        shifted_digits.extend_from_slice(&self.digits);
        MegaUInt {
            digits: shifted_digits,
        }
    }

    fn multiply_by_digit(&self, digit: i8) -> MegaUInt {
        let mut result = Vec::new();
        let mut carry = 0;

        for &d in &self.digits {
            let product = d * digit + carry;
            result.push(product % 10);
            carry = product / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        MegaUInt { digits: result }
    }
}

pub fn mega_uint_factorial(n: u32) -> MegaUInt {
    let mut result = MegaUInt::one();
    for i in 2..=n {
        result = result.multiply(&mega_int_from_string(&i.to_string()));
    }
    result
}

impl Display for MegaUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .rev()
                .map(|d| d.to_string())
                .collect::<String>()
        )
    }
}
