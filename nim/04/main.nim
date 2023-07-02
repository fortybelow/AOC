import system/io
import strformat
import os

type Entry = object
  character: char

iterator iterate(tower: seq[seq[Entry]]): Entry =
    for r in 0..<len(tower):
        for c in 0..<len(tower[r]):
            yield tower[r][c]

if os.paramCount() != 1:
    echo fmt"Usage: {os.paramStr(0)} <input>"
    quit(1)

let input_path: string = os.paramStr(1)
echo fmt"Opening {input_path}"

var stream = open(input_path, fmRead)

var previous_character: char = '?'
var char_number = 0
var line_number = 0

var entries: seq[Entry]
entries = @[]

# Sequence of sequences of Entry
var tower: seq[seq[Entry]] = @[]
var concurrent_spaces = 0

while true:
    let current_character: char = stream.readChar()
    line_number += (if current_character == '\n': 1 else: 0)
    
    if current_character == '\n':
        if len(entries) >= 0:
            tower.add(entries)
            entries = newSeq[Entry]()
        if current_character == previous_character:
            echo fmt"Found end of tower {ord(previous_character)} {ord(current_character)}. Note line number {line_number}"
            break

        concurrent_spaces = 0

    elif current_character == '[':
        let entry = stream.readChar()
        let entry_end = stream.readChar()
        entries.add(Entry(character: entry))
        concurrent_spaces = 0

    else:
        concurrent_spaces += 1

    if concurrent_spaces == 3:
        entries.add(Entry(character: ' '))
        concurrent_spaces = 0

    previous_character = current_character
    char_number += 1

# Pop last two entries ...
tower.pop()

echo "Tower:", tower
echo ".... or ...."
for i in 0 ..< len(tower):
    echo tower[i]
echo "..."

# Transpose tower
var transposed_tower: seq[seq[Entry]] = @[]
for i in 0 ..< tower[0].len:
    var transposed_line: seq[Entry] = @[]
    for j in 0 ..< tower.len:
        transposed_line.add(tower[j][i])
    transposed_tower.add(transposed_line)


# echo "Transposed:", transposed_tower
# for i in 0 ..< len(tower):
#     echo fmt"Row {i}: {tower[i]}"

