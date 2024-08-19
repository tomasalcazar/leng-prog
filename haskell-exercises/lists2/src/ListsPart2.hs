module ListsPart2 (Bit(..), bitAt, charToBits, bits, queens) where

import Data.Char(ord)
import Data.Bits(testBit)

data Bit = F | T  deriving (Eq, Show, Enum, Read)
type Bits = [Bit]

bitAt :: Int -> Char -> Bit
bitAt n c = if testBit (ord c) (7-n) then T else F

charToBits :: Char -> Bits
charToBits c = [bitAt n c | n <- [0..7]]

bits :: String -> Bits
bits = concatMap charToBits

type Solution = [Int]

queens :: Int -> [Solution]
queens n = placeQueens n
  where
    placeQueens 0 = [[]]
    placeQueens k = [q:qs | qs <- placeQueens (k-1), q <- [1..n], safe q qs 1]

    safe _ [] _ = True
    safe q (q':qs) n = q /= q' && abs (q - q') /= n && safe q qs (n + 1)
