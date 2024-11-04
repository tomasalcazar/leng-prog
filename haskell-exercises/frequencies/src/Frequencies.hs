module Frequencies  (Frequency, frequencyMap, frequencies, insert, insertionSort) where

import Data.Map (Map)
import qualified Data.Map as Map
import Data.Tuple (swap)
import Trie

type Frequency = (Int, Char)

insertInTrie :: Char -> Trie (Char, Int) -> Trie (Char, Int)
insertInTrie c (Leaf (ch, freq))
    | c == ch   = Leaf (ch, freq + 1)
    | otherwise = Leaf (c, 1) :-: Leaf (ch, freq)
insertInTrie c (l :-: r)
    | c <= getChar l = insertInTrie c l :-: r
    | otherwise      = l :-: insertInTrie c r

getChar :: Trie (Char, Int) -> Char
getChar (Leaf (ch, _)) = ch
getChar ((l :-: _) :-: _) = getChar l

buildTrie :: String -> Trie (Char, Int)
buildTrie = foldr insertInTrie (Leaf (' ', 0))

trieToMapList :: Trie (Char, Int) -> [(Char, Int)]
trieToMapList trie = map (\(v, bits) -> v) $ toList trie

frequencyMap :: String -> [(Char, Int)]
frequencyMap str = trieToMapList $ buildTrie str

insert :: (Ord a) => a -> [a] -> [a]
insert x [] = [x]
insert x (y:ys)
    | x <= y    = x : y : ys
    | otherwise = y : insert x ys

insertionSort :: (Ord a) => [a] -> [a]
insertionSort = foldr insert []

frequencies :: String -> [Frequency]
frequencies str = insertionSort . map swap $ frequencyMap str
  where
    swap (a, b) = (b, a)
