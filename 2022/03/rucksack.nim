import strutils
import sequtils
import std/setutils

type
  Rucksack* = object
    contents*: string

# Method for Rucksack
proc first_compartment*(r: Rucksack): string = 
  r.contents[0 .. r.contents.len div 2 - 1]

proc second_compartment*(r: Rucksack): string =
  r.contents[r.contents.len div 2 .. r.contents.len - 1]

proc common_letters*[T,U](s1: T, s2: U): auto =
  let intersection = s1.filterIt(it in s2)
  intersection.toSet.toSeq

proc priority*(ch: char): int =
  case ch
  of 'A'..'Z': ord(ch) - ord('A') + 26 + 1
  of 'a'..'z': ord(ch) - ord('a') + 1
  else: 0

