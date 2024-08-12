module Fraction (Fraction, add, sub, mul, divide, hcf) where

type Fraction = (Int, Int)

-- Helper function to simplify fractions
simplify :: Fraction -> Fraction
simplify (num, den) = (num `div` h, den `div` h)
  where h = hcf (abs num) (abs den)

-- Implement the `add` Function
add :: Fraction -> Fraction -> Fraction
add (n1, d1) (n2, d2) = (n1 * d2 + n2 * d1, d1 * d2)

-- Implement the `sub` Function
sub :: Fraction -> Fraction -> Fraction
sub (n1, d1) (n2, d2) = (n1 * d2 - n2 * d1, d1 * d2)

-- Implement the `mul` Function
mul :: Fraction -> Fraction -> Fraction
mul (n1, d1) (n2, d2) = (n1 * n2, d1 * d2)

-- Implement the `divide` Function
divide :: Fraction -> Fraction -> Fraction
divide (n1, d1) (n2, d2) = (n1 * d2, d1 * n2)

-- Implement the `hcf` Function
hcf :: Int -> Int -> Int /highest common factor/
hcf a 0 = a
hcf a b = hcf b (a `mod` b)
