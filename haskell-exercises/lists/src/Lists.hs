module Lists (
    member, union, intersection, difference,
    insert, insertionSort,
    binaryToDecimal, toDecimal, toDec, decimal,
    firsts, binaryAdd, merge, mergeSort
) where  -- Fix the misplaced closing parenthesis here

import Data.Char (digitToInt, intToDigit)

-- List as Sets

member :: Int -> [Int] -> Bool
member :: Int -> [Int] -> Bool
member _ []      = False
member e (x:xs)  = e == x || member e xs

union :: [Int] -> [Int] -> [Int]
union :: [Int] -> [Int] -> [Int]
union [] ys     = ys
union (x:xs) ys
union (x:xs) ys
  | member x ys = union xs ys
  | otherwise   = x : union xs ys

intersection :: [Int] -> [Int] -> [Int]
intersection [] _ = []
intersection (x:xs) ys
  | member x ys = x : intersection xs ys
  | otherwise   = intersection xs ys
intersection :: [Int] -> [Int] -> [Int]
intersection [] _ = []
intersection (x:xs) ys
  | member x ys = x : intersection xs ys
  | otherwise   = intersection xs ys

difference :: [Int] -> [Int] -> [Int]  -- Example: difference [1, 2, 3, 4] [3, 4, 5, 6] = [1, 2]
difference [] _ = []
difference (x:xs) ys
  | member x ys = difference xs ys
  | otherwise   = x : difference xs ys

-- Insertion Sort

insert :: Int -> [Int] -> [Int]
insert e [] = [e]
insert e (x:xs)
  | e <= x    = e : x : xs
  | otherwise = x : insert e xs
insert :: Int -> [Int] -> [Int]
insert e [] = [e]
insert e (x:xs)
  | e <= x    = e : x : xs
  | otherwise = x : insert e xs

insertionSort :: [Int] -> [Int]
insertionSort [] = []
insertionSort (x:xs) = insert x (insertionSort xs)

-- Numeral Systems

binaryToDecimal :: [Int] -> Int
binaryToDecimal = foldl (\acc x -> acc * 2 + x) 0

binaryToDecimal = foldl (\acc x -> acc * 2 + x) 0

toDecimal :: Int -> [Int] -> Int
toDecimal base = foldl (\acc x -> acc * base + x) 0

toDec :: Int -> String -> Int
toDec base s = toDecimal base (map digitToInt s)

decimal :: Int -> String -> Int
decimal base s = sum [digitToInt x * base^i | (x, i) <- zip (reverse s) [0..]]

-- Firsts Elements of a List

firsts :: [a] -> [[a]]
firsts [] = []
firsts xs = [take n xs | n <- [1..length xs]]

-- Binary Operations – BONUS exercise

binaryAdd :: String -> String -> String
binaryAdd "" "" = "0"  -- Handle the case where both inputs are empty
binaryAdd xs ys = reverse (addBinary (reverse xs) (reverse ys) '0')

addBinary :: String -> String -> Char -> String
addBinary [] [] carry = if carry == '1' then [carry] else []
addBinary (x:xs) [] carry = addBinary (x:xs) "0" carry
addBinary [] (y:ys) carry = addBinary "0" (y:ys) carry
addBinary (x:xs) (y:ys) carry
  | sum == 2  = '0' : addBinary xs ys '1'
  | sum == 3  = '1' : addBinary xs ys '1'
  | otherwise = intToDigit sum : addBinary xs ys '0'
  where sum = digitToInt x + digitToInt y + digitToInt carry

-- Merge Sort

merge :: [Int] -> [Int] -> [Int]
merge [] ys = ys
merge xs [] = xs
merge (x:xs) (y:ys)
  | x <= y    = x : merge xs (y:ys)
  | otherwise = y : merge (x:xs) ys

mergeSort :: [Int] -> [Int]
mergeSort [] = []
mergeSort [x] = [x]
mergeSort xs = merge (mergeSort left) (mergeSort right)
  where
    (left, right) = splitAt (length xs `div` 2) xs
