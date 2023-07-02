import strutils

# Enum of char
type
  Play = enum
    Rock, Paper, Scissors
  Suggestion = enum
    Lose, Draw, Win

proc losing(against: Play): Play =
  case against
  of Rock: Scissors
  of Paper: Rock
  of Scissors: Paper

proc winning(against: Play): Play =
  case against
  of Rock: Paper
  of Paper: Scissors
  of Scissors: Rock

proc score(play: Play): int =
  case play
  of Play.Rock: 1
  of Play.Paper: 2
  of Play.Scissors: 3

proc score(suggestion: Suggestion): int =
  case suggestion
  of Suggestion.Lose: 0
  of Suggestion.Draw: 3
  of Suggestion.Win: 6

# Convert char to either Play or Suggestion
proc toPlay(c: char): Play =
  case c
  of 'A': Rock
  of 'B': Paper
  of 'C': Scissors
  else: raise newException(ValueError, "Invalid char: " & $c)

proc toSuggestion(c: char): Suggestion = 
  case c
  of 'X': Lose
  of 'Y': Draw
  of 'Z': Win
  else: raise newException(ValueError, "Invalid char: " & $c)

iterator parse(file: string): tuple[opponent: Play, suggestion: Suggestion] =
  for line in file.lines():
    var suggestions = line.split(" ")
    yield (suggestions[0][0].toPlay(), suggestions[1][0].toSuggestion())

proc move(opponent: Play, suggested: Suggestion): Play =
  result = case suggested:
    of Draw: opponent
    of Lose: losing(opponent)
    of Win: winning(opponent)

proc evaluate(opponent: Play, suggested: Suggestion): int =
  score(move(opponent, suggested)) + score(suggested)

var totalScore = 0
for (opponent, suggested) in parse("input.txt"):
  echo "Opponent '", opponent, "' suggestion '", suggested, "' -> Plays ", move(opponent, suggested), " -> Score ", evaluate(opponent, suggested)
  totalScore += evaluate(opponent, suggested)

echo "Strategy score: ", totalScore
