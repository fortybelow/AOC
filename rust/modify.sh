#!/bin/bash

#use getopt to check if --delete or --add is set.

options_description='''
Usage: $0 <day>
    -a, --add       Add a new day
    -d, --delete    Delete a day
'''

function day_description {
day=$1
echo "
[[bin]]
name = \"day$day\"
path = \"src/day$day/day$day.rs\"
args = [\"src/day$day/day$day.txt\"]
"
}


options=$(getopt -o a:d: --long add:,delete: -- "$@")
if [[ $? != 0 ]] || [[ $# -eq 0 ]]; then echo "missing options${options_description}" >&2 ; exit 1 ; fi

eval set -- "$options"
while true; do
    case "$1" in
        -a | --add )
            shift;
            day=$1;
            day_already_exists=$(grep -c "day$day" $(dirname $(realpath $0))/Cargo.toml)

            if [ $day_already_exists -gt 0 ]; then
                echo "Day $day already exists"
                exit 1
            fi

            if [ -z "$day" ] || ! [[ "$day" =~ ^[0-9]+$ ]]; then
                echo "Missing day number"
                exit 1
            fi

            previous_toml=$(cat Cargo.toml)
            new_day_toml=$(day_description $day)
            echo "$new_day_toml" >> Cargo.toml
            mkdir -p src/day$day
            touch src/day$day/day$day.rs
            touch src/day$day/day$day.txt

            # Show diff of toml
            echo "Diff of Cargo.toml"
            diff <(echo "$previous_toml") Cargo.toml

            shift
            ;;
        -d | --delete )
            shift;
            day=$1;

            if [ -z "$day" ] || ! [[ "$day" =~ ^[0-9]+$ ]]; then
                echo "Missing day number"
                exit 1
            fi
            
            TARGET="name = \"day$day\""
            echo "Removing target $TARGET"

            awk -v tgt="$TARGET" '
                # Store the lines in an array
                {a[NR]=$0}

                # If the current line matches the target
                $0 ~ tgt {del[NR-1]; del[NR]; del[NR+1]; del[NR+2]}

                # At the end of processing
                END {
                    # Print the lines that have not been marked for deletion
                    for(i=1; i<=NR; i++)
                        if (!(i in del))
                            print a[i]
                }
            ' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

            if [ -d src/day$day ]; then
                echo "Removing src/day$day"
                rm -rf src/day$day
            fi

            shift
            ;;
        -- ) shift; break ;;
        * ) break ;;
    esac
done
