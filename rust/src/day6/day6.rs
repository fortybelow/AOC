// Purpose: Solve day 6 tasks
use std::collections::HashMap;

struct Graph<'a> {
    nodes: HashMap<&'a str, Vec<&'a str>>,
    reverse: HashMap<&'a str, &'a str>,
}

impl <'a> Graph<'a> {
    fn add_edge(&mut self, a: &'a str, b: &'a str) {
        self.nodes.entry(a).or_default().push(b);
        self.reverse.insert(b, a);
    }
}

impl Graph<'_> {
    fn new() -> Graph<'static> {
        Graph {
            nodes: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    fn total_orbits(&self) -> usize {
        let mut total = 0;
        let mut queue = vec![("COM", 0)];
        while let Some((node, depth)) = queue.pop() {
            total += depth;
            if let Some(children) = self.nodes.get(node) {
                for child in children {
                    queue.push((child, depth + 1));
                }
            }
        }
        total
    }

    fn distance(&self, a: &str, b: &str) -> usize {
        let mut queue = vec![(a, 0)];
        let mut visited = HashMap::new();
        while let Some((node, depth)) = queue.pop() {
            if node == b {
                return depth - 2;
            }

            // Add children and parent to queue (Breath-first search)
            if let Some(children) = self.nodes.get(node) {
                for child in children {
                    if visited.insert(child, depth + 1).is_none() {
                        queue.push((child, depth + 1));
                    }
                }
            }
            if let Some(parent) = self.reverse.get(node) {
                if visited.insert(parent, depth + 1).is_none() {
                    queue.push((parent, depth + 1));
                }
            }
        }
        panic!("Could not find path from {} to {}", a, b);
    }
}

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        let input = std::fs::read_to_string(arg).expect("Could not read file");
        let input: Vec<(&str, &str)> = input
            .lines()
            .map(|line| {
                let mut split = line.split(')');
                let a = split.next().unwrap();
                let b = split.next().unwrap();
                assert!(split.next().is_none());
                (a, b)
            })
            .collect();

        let mut graph = Graph::new();
        for (a, b) in &input {
            graph.add_edge(a, b);
        }

        println!("Total orbits: {}", graph.total_orbits());
        println!("Distance: {}", graph.distance("YOU", "SAN"));
    } else {
        println!("Expected path to input as argument");
    }
}
