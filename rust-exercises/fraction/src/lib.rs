type Fraction = (i32, i32);

/// Add 2 fractions
pub fn add((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let numerator = n1 * d2 + n2 * d1;
    let denominator = d1 * d2;
    simplify(numerator, denominator)
}

/// Subtract 2 fractions
pub fn sub((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let numerator = n1 * d2 - n2 * d1;
    let denominator = d1 * d2;
    simplify(numerator, denominator)
}

/// Multiply 2 fractions
pub fn mul((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let numerator = n1 * n2;
    let denominator = d1 * d2;
    simplify(numerator, denominator)
}

/// Divide 2 fractions
pub fn divide((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let numerator = n1 * d2;
    let denominator = d1 * n2;
    simplify(numerator, denominator)
}

/// Calculate the Highest common factor (greatest common divisor) between 2 numbers
pub fn hcf(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        hcf(b, a % b)
    }
}

/// Create a fraction simplifying the arguments by their highest common factor (HCF)
pub fn simplify(n: i32, d: i32) -> Fraction {
    let divisor = hcf(n.abs(), d.abs());
    (n / divisor, d / divisor)
}
