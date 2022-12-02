import os
import strutils
import sequtils
import std/tables
import std/algorithm

iterator parse(filename: string): tuple[id: int, presents: int] =
    let file = open(filename, FileMode.fmRead)
    defer: file.close()

    var accumulator: tuple[id: int, presents: int] = (0, 0)

    for line in file.lines():
        # Line is a single integer or newline
        if line.len == 0:
            yield accumulator
            accumulator = (accumulator[0] + 1, 0)
        else:
            accumulator[1] += parseInt(line.strip())

# If no file is given, complain
if os.paramCount() != 2:
    echo "Please provide a filename and a number of elves"
    quit(1)

type
    Elf = object
        id: int
        presents: int

    ElfList = seq[Elf]

    ElfContext = object
        elves: ElfList
        pointers: seq[ptr Elf]

method size(self: ElfContext): int {.base.} =
    self.elves.len

iterator each(self: ElfContext): Elf =
    for elf in self.elves:
        yield elf

proc initElfContext(filename: string): ElfContext =
    var
        elfContext = ElfContext(
            elves: @[],
            pointers: @[]
        )

    for (id, presents) in parse(filename):
        elfContext.elves.add(Elf(id: id, presents: presents))
    for elf in elfContext.elves:
        elfContext.pointers.add(addr elfContext.elves[^1])

    elfContext.pointers.sort(proc (x, y: ptr Elf): int = cmp(x.presents, y.presents))

    return elfContext

var
    elfContext = initElfContext(os.paramStr(1))
    elfCount = parseInt(os.paramStr(2))
    

# Print out the top 3 elves
var totalPresents = 0

for index in 1..elfCount:
    let elf = elfContext.pointers[^index]
    echo "Elf ", elf.id, " has ", elf.presents, " presents"
    totalPresents += elf.presents

echo "Total presents: ", totalPresents


