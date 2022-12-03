import os
import sequtils
import strutils
import rucksack

proc priorities(inputfile: string) =
  var total = 0
  
  for line in lines(open(inputfile)):
    var sack = Rucksack(contents: line)
   
    let
      fst = sack.first_compartment()
      snd = sack.second_compartment()
  
    var common = rucksack.common_letters(fst, snd)
    let sum = common.mapIt(rucksack.priority(it)).foldl(a + b)
    total += sum
    echo fst, " ", snd, " '", common, "'", " ", sum
  
  echo "Total: ", total

proc badges(inputfile: string) =
  var total = 0
  var group = newSeq[string]()
  for line in lines(open(inputfile)):
    group.add(line)
    if group.len == 3:
      let
        rucksacks = group.mapIt(Rucksack(contents: it))
        common = rucksack.common_letters(
          rucksack.common_letters(rucksacks[0].contents, rucksacks[1].contents),
          rucksacks[2].contents
        )
        sum = common.mapIt(rucksack.priority(it)).foldl(a + b)

      total += sum
      echo rucksacks[0].contents, " ", rucksacks[1].contents, " ", rucksacks[2].contents, " '", common, "'", " ", sum, " ", total
      group = newSeq[string]()
   
proc main() =
  let
    argc = os.paramCount()
  
  if argc != 2:
    echo "Usage: ", os.paramStr(0), " <part> <filename>"
    quit(0)

  if os.paramStr(1).toLower in ["1", "one"]:
    priorities(os.paramStr(2))
  else:
    badges(os.paramStr(2))

when is_main_module:
  main()
  
