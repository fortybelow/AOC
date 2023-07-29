#!/bin/bash

#use getopt to check if --delete or --add is set.

options_description='''
Usage: $0 <day>
    -a, --add       Add a new day
    -d, --delete    Delete a day
'''

new_day_description='''
[[bin]]
name = "day'''$day'''"
path = "src/day'''$day'''/day'''$day'''.rs"
args = ["src/day'''$day'''/day'''$day'''.txt"]
'''

options=$(getopt -o ad --long add,delete -- "$@")
if [ $? != 0 ] ; then echo "missing options\n${options_descrption}" >&2 ; exit 1 ; fi

eval set -- "$options"
while true; do
    case "$1" in
        -a | --add )
            day_already_exists=$(grep -c "day$day" $(dirname $(realpath $0))/Cargo.toml)

            if [ $day_already_exists -gt 0 ]; then
                echo "Day $day already exists"
                exit 1
            fi

            shift;
            day=$1;

            if [ -z "$day" ]; then
                echo "Missing day number"
                exit 1
            fi

            echo "$new_day_description" >> Cargo.toml
            mkdir -p src/day$day
            touch src/day$day/day$day.rs
            touch src/day$day/day$day.txt

            shift;
        -d | --delete )
            shift;
            day=$1;

            if [ -z "$day" ]; then
                echo "Missing day number"
                exit 1
            fi

            # remove day description from Cargo.toml
            # Look for name = "day$day" and delete the next 3 lines and previous line.
            sed -i "/name = \"day$day\"/,+3d" Cargo.toml
            sed -i "/name = \"day$day\"/,+1d" Cargo.toml
            rm -rf src/day$day

            ;;
        -- ) shift; break ;;
        * ) break ;;
    esac
done
