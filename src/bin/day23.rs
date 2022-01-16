use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use itertools::{iproduct, Itertools};

use aoc2021::util::input_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct State {
    positions: [[(u8, u8); 2]; 4],
}

fn main() -> Result<()> {
    let state = parse_input()?;

    let least_energy = find_least_energy(state).unwrap();
    dbg!(least_energy);

    Ok(())
}

fn find_least_energy(start: State) -> Option<u32> {
    // https://en.wikipedia.org/w/index.php?title=A*_search_algorithm&oldid=1060160033

    fn heuristic(state: &State) -> u32 {
        iproduct!([Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D], 0..=1)
            .map(|(pod, index)| {
                if state.is_in_target_room(pod, index) {
                    0
                } else {
                    pod.energy() * manhattan_distance(state[pod][index], (2, pod.target_room_col()))
                }
            })
            .sum()
    }

    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, heuristic(&start));

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by_key(|state| f_score.get(state).unwrap_or(&u32::MAX))
            .unwrap();

        // Check whether goal
        if current.is_final() {
            return Some(g_score[&current]);
        }

        open_set.remove(&current);

        for (neighbour, energy) in current.next() {
            let tentative_g_score = g_score[&current] + energy;
            if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&u32::MAX) {
                // This path to neighbor is better than any previous one. Record it!
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + heuristic(&neighbour));
                open_set.insert(neighbour);
            }
        }
    }

    None
}

fn manhattan_distance(source: (u8, u8), dest: (u8, u8)) -> u32 {
    let vert = (source.0 as i16 - dest.0 as i16).abs() as u32;
    let hor = (source.1 as i16 - dest.1 as i16).abs() as u32;
    vert + hor
}

impl Amphipod {
    fn energy(&self) -> u32 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn target_room_col(&self) -> u8 {
        match self {
            Amphipod::A => 3,
            Amphipod::B => 5,
            Amphipod::C => 7,
            Amphipod::D => 9,
        }
    }
}

impl State {
    fn is_final(&self) -> bool {
        self[Amphipod::A][0].1 == Amphipod::A.target_room_col()
            && self[Amphipod::A][1].1 == Amphipod::A.target_room_col()
            && self[Amphipod::B][0].1 == Amphipod::B.target_room_col()
            && self[Amphipod::B][1].1 == Amphipod::B.target_room_col()
            && self[Amphipod::C][0].1 == Amphipod::C.target_room_col()
            && self[Amphipod::C][1].1 == Amphipod::C.target_room_col()
            && self[Amphipod::D][0].1 == Amphipod::D.target_room_col()
            && self[Amphipod::D][1].1 == Amphipod::D.target_room_col()
    }

    fn is_in_room(&self, pod: Amphipod, index: usize) -> bool {
        let row = self[pod][index].0;
        row == 2 || row == 3
    }

    fn is_in_target_room(&self, pod: Amphipod, index: usize) -> bool {
        let col = self[pod][index].1;
        self.is_in_room(pod, index) && col == pod.target_room_col()
    }

    fn next(&self) -> impl Iterator<Item = (State, u32)> + '_ {
        let occupant = |pos: (u8, u8)| {
            for pod in [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D] {
                let pod_positions = &self[pod];
                for position in pod_positions {
                    if position == &pos {
                        return Some(pod);
                    }
                }
            }
            None
        };

        let path_positions = |source: (u8, u8), dest: (u8, u8)| {
            let (source_row, source_col) = source;
            let (dest_row, dest_col) = dest;

            if source_row <= dest_row {
                // From corridor into room
                if dest_col <= source_col {
                    // Moving left
                    (dest_col..=source_col)
                        .rev()
                        .map(|col| (source_row, col))
                        .chain((source_row..=dest_row).map(|row| (row, dest_col)))
                        .collect_vec()
                        .into_iter()
                } else {
                    // Moving Right
                    (source_col..=dest_col)
                        .map(|col| (source_row, col))
                        .chain((source_row..=dest_row).map(|row| (row, dest_col)))
                        .collect_vec()
                        .into_iter()
                }
            } else {
                // From room into corridor
                if dest_col <= source_col {
                    // Moving left
                    (dest_row..=source_row)
                        .rev()
                        .map(|row| (row, source_col))
                        .chain((dest_col..=source_col).rev().map(|col| (dest_row, col)))
                        .collect_vec()
                        .into_iter()
                } else {
                    // Moving Right
                    (dest_row..=source_row)
                        .rev()
                        .map(|row| (row, source_col))
                        .chain((source_col..=dest_col).rev().map(|col| (dest_row, col)))
                        .collect_vec()
                        .into_iter()
                }
            }
            .skip(1)
        };

        let is_path_clear = move |source: (u8, u8), dest: (u8, u8)| {
            path_positions(source, dest).all(|pos| occupant(pos).is_none())
        };

        iproduct!([Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D], 0..=1).flat_map(
            move |(pod, index)| {
                let current_position = self[pod][index];

                let target_positions = if self.is_in_room(pod, index) {
                    if self.is_in_target_room(pod, index)
                        && (current_position.0 == 3
                            || occupant((3, current_position.1)).unwrap() == pod)
                    {
                        vec![]
                    } else {
                        // Enumerate all moves into the corridor

                        // Moves to the left end
                        let left = path_positions(current_position, (1, 1))
                            .take_while(|pos| occupant(*pos).is_none())
                            .filter(|pos| ![3, 5, 7, 9].contains(&pos.1))
                            .map(|pos| {
                                (
                                    pos,
                                    manhattan_distance(current_position, pos) * pod.energy(),
                                )
                            });

                        // Moves to the right end
                        let right = path_positions(current_position, (1, 11))
                            .take_while(|pos| occupant(*pos).is_none())
                            .filter(|pos| ![3, 5, 7, 9].contains(&pos.1))
                            .map(|pos| {
                                (
                                    pos,
                                    manhattan_distance(current_position, pos) * pod.energy(),
                                )
                            });

                        left.chain(right).collect_vec()
                    }
                } else {
                    // Move into the target room, if possible

                    let dest_col = match pod {
                        Amphipod::A => 3,
                        Amphipod::B => 5,
                        Amphipod::C => 7,
                        Amphipod::D => 9,
                    };

                    if is_path_clear(current_position, (3, dest_col)) {
                        vec![(
                            (3, dest_col),
                            manhattan_distance(current_position, (3, dest_col)) * pod.energy(),
                        )]
                    } else if is_path_clear(current_position, (2, dest_col))
                        && occupant((3, dest_col)).unwrap() == pod
                    {
                        vec![(
                            (2, dest_col),
                            manhattan_distance(current_position, (2, dest_col)) * pod.energy(),
                        )]
                    } else {
                        vec![]
                    }
                };

                target_positions.into_iter().map(move |(pos, energy)| {
                    let mut new_state = *self;
                    new_state[pod][index] = pos;
                    (new_state, energy)
                })
            },
        )
    }
}

fn parse_input() -> Result<State> {
    let pods: Result<Vec<_>> = input_lines()?
        .skip(2)
        .take(2)
        .enumerate()
        .map(|(index, line)| -> Result<[(Amphipod, (u8, u8)); 4]> {
            let line = line?;

            assert!(line.is_ascii());

            let row = index as u8 + 2;

            Ok([
                ((&line[3..4]).parse()?, (row, 3)),
                ((&line[5..6]).parse()?, (row, 5)),
                ((&line[7..8]).parse()?, (row, 7)),
                ((&line[9..10]).parse()?, (row, 9)),
            ])
        })
        .collect();
    let pods = pods?.into_iter().flatten().sorted().collect_vec();

    let mut state = State::default();
    for (key, group) in &pods.into_iter().group_by(|element| element.0) {
        let positions: (_, _) = group
            .collect_tuple()
            .context("Invalid number of amphipods")?;
        state[key][0] = positions.0 .1;
        state[key][1] = positions.1 .1;
    }

    Ok(state)
}

impl FromStr for Amphipod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Amphipod::A,
            "B" => Amphipod::B,
            "C" => Amphipod::C,
            "D" => Amphipod::D,
            _ => bail!("Invalid amphipod"),
        })
    }
}

impl Index<Amphipod> for State {
    type Output = [(u8, u8); 2];

    fn index(&self, index: Amphipod) -> &Self::Output {
        &self.positions[index as usize]
    }
}

impl IndexMut<Amphipod> for State {
    fn index_mut(&mut self, index: Amphipod) -> &mut Self::Output {
        &mut self.positions[index as usize]
    }
}
