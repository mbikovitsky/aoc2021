use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let graph = parse_input()?;

    let paths = count_paths(&graph);
    dbg!(paths);

    let paths2 = count_paths2(&graph);
    dbg!(paths2);

    Ok(())
}

fn count_paths(graph: &HashMap<String, Vec<String>>) -> u64 {
    fn backtrack<'a>(
        graph: &'a HashMap<String, Vec<String>>,
        current: &str,
        visited: &mut HashSet<&'a str>,
    ) -> u64 {
        if current == "end" {
            return 1;
        }

        let mut count = 0;

        for neighbour in graph.get(current).unwrap() {
            if visited.contains(neighbour.as_str()) {
                continue;
            }

            if neighbour.chars().all(|c| c.is_lowercase()) {
                visited.insert(neighbour);
            }

            count += backtrack(graph, neighbour, visited);

            visited.remove(neighbour.as_str());
        }

        count
    }

    let mut visited = HashSet::from(["start"]);
    backtrack(graph, "start", &mut visited)
}

fn count_paths2(graph: &HashMap<String, Vec<String>>) -> u64 {
    fn backtrack<'a>(
        graph: &'a HashMap<String, Vec<String>>,
        current: &str,
        visited: &mut HashSet<&'a str>,
        small_cave_visited_twice: bool,
    ) -> u64 {
        if current == "end" {
            return 1;
        }

        let mut count = 0;

        for neighbour in graph.get(current).unwrap() {
            let is_small_cave = neighbour.chars().all(|c| c.is_lowercase());

            let neighbour_visited = visited.contains(neighbour.as_str());

            let mut inserted = false;

            let mut small_cave_visited_twice = small_cave_visited_twice;

            if is_small_cave {
                if neighbour_visited {
                    if small_cave_visited_twice || neighbour == "start" {
                        continue;
                    }
                    small_cave_visited_twice = true;
                } else {
                    visited.insert(neighbour);
                    inserted = true;
                }
            } else {
                assert!(!neighbour_visited);
            }

            count += backtrack(graph, neighbour, visited, small_cave_visited_twice);

            if inserted {
                visited.remove(neighbour.as_str());
            }
        }

        count
    }

    let mut visited = HashSet::from(["start"]);
    backtrack(graph, "start", &mut visited, false)
}

fn parse_input() -> Result<HashMap<String, Vec<String>>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    let mut add_edge = |a: &str, b: String| {
        if let Some(neighbours) = result.get_mut(a) {
            neighbours.push(b);
        } else {
            result.insert(a.to_string(), vec![b]);
        }
    };

    for line in input_lines()? {
        let line = line?;

        let vertices = line.split_once('-').context("Missing delimiter")?;
        let a = vertices.0.to_string();
        let b = vertices.1.to_string();

        add_edge(&a, b.clone());
        add_edge(&b, a);
    }

    Ok(result)
}
