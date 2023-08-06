use std::collections::HashMap;
use std::collections::HashSet;

macro_rules! show_me {
    ($e:expr) => {
        {
            let val = $e;
            println!("{:?}", val);
            val
        }
    };
}

#[derive(Debug)]
struct Path {
    steps: Vec<Step>,
}

type Step = (Direction, isize);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// char to Direction
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

fn compute(input: &str) -> (isize, isize) {
    let paths: Vec<Path> = input
        .lines()
        .map(|line| {
            let steps: Vec<Step> = line
                .split(',')
                .map(|step| {
                    let direction = step.chars().next().unwrap().into();
                    let distance = step[1..].parse().unwrap();
                    (direction, distance)
                })
                .collect();
            Path { steps }
        })
        .collect();

    let mut grid = HashMap::new();
    let mut intersections = HashSet::new();
    let mut steps = HashMap::new();
    
    for (i, path) in paths.iter().enumerate() {
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut step_count: isize = 0;
        for (direction, distance) in &path.steps {
            for _ in 0..*distance {
                match direction {
                    Direction::Up => y = y + 1,
                    Direction::Down => y = y - 1,
                    Direction::Left => x = x - 1,
                    Direction::Right => x = x + 1,
                }
                
                step_count += 1;

                let point: (isize, isize) = (x, y);
                if steps.contains_key(&point) {
                    let v: &mut Vec<(isize, isize)> = steps.get_mut(&point).unwrap();
                    if v.iter().any(|(wire, _)| *wire == i as isize) {
                        continue;
                    }
                    v.push((i as isize, step_count));
                } else {
                    steps.insert(point, vec![(i as isize, step_count)]);
                }

                if grid.contains_key(&point) {
                    intersections.insert(point);
                    println!("Intersecting wires: {:?}", steps.get(&point));
                } else {
                    grid.insert(point, i);
                }
            }
        }
    }

    let mut min_distance = isize::max_value();
    let mut min_steps = isize::max_value();
    let mut closest_intersection = None;
    let mut closest_steps = None;

    for point in intersections {
        let point = show_me!(point);
        let manhattan_distance = point.0.abs() + point.1.abs();
        if manhattan_distance < min_distance {
            min_distance = manhattan_distance;
            closest_intersection = Some(point);
        }

        let mut total_steps = 0;
        for (_, steps) in steps.get(&point).unwrap() {
            total_steps += steps;
        }

        if total_steps < min_steps {
            min_steps = total_steps;
            closest_steps = Some(point);
            println!("New closest steps: {:?} - All {:?}", closest_steps, steps.get(&point).unwrap());
        }
    }

    println!("Closest {:?} with distance {}", closest_intersection, min_distance);
    println!("Closest {:?} with steps {}", closest_steps, min_steps);
    
    (min_distance, min_steps)
}

fn main() {
    let input = include_str!("day3.txt");
    let (_distance, _steps) = compute(input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let (distance, steps) = compute(input);
        assert_eq!(distance, 6);
        assert_eq!(steps, 30);
    }

    #[test]
    fn test2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let (distance, steps) = compute(input);
        assert_eq!(distance, 159);
        assert_eq!(steps, 610);
    }

    #[test]
    fn test3() {
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let (distance, steps) = compute(input);
        assert_eq!(distance, 135);
        assert_eq!(steps, 410);
    }
}
