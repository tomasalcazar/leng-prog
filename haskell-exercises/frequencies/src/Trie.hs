module Trie  (Trie(..), left, right, find, decode, toList) where

import Bit

data Trie a = Leaf a | Trie a :-: Trie a deriving (Eq, Show)

left :: Trie a -> Trie a
left (l :-: _) = l
left _ = error "Left called on a Leaf"

right :: Trie a -> Trie a
right (_ :-: r) = r
right _ = error "Right called on a Leaf"

find :: Bits -> Trie a -> a
find [] (Leaf x) = x
find (F:bs) (l :-: _) = find bs l
find (T:bs) (_ :-: r) = find bs r
find _ _ = error "Invalid Bits or Trie structure"

decode :: Bits -> Trie Char -> String
decode [] _ = []
decode bits trie = decodeHelper bits trie trie

decodeHelper :: Bits -> Trie Char -> Trie Char -> String
decodeHelper [] _ (Leaf c) = [c]
decodeHelper (F:bs) (l :-: _) originalTrie = decodeHelper bs l originalTrie
decodeHelper (T:bs) (_ :-: r) originalTrie = decodeHelper bs r originalTrie
decodeHelper bs _ originalTrie = decodeHelper bs originalTrie originalTrie

toList :: Trie a -> [(a, Bits)]
toList (Leaf x) = [(x, [])]
toList (l :-: r) = [(v, F:bs) | (v, bs) <- toList l] ++ [(v, T:bs) | (v, bs) <- toList r]
