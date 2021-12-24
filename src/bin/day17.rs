use std::io::Read;

use anyhow::{Context, Result};
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::util::input_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

fn main() -> Result<()> {
    let target_area = parse_input()?;

    let all_velocities = find_all_initial_velocities(target_area).collect_vec();

    let max_y_velocity = all_velocities.iter().max_by_key(|(_, vy)| vy).unwrap().1;
    let apogee = calculate_apogee(max_y_velocity);
    dbg!(apogee);

    dbg!(all_velocities.len());

    Ok(())
}

fn find_all_initial_velocities(target: TargetArea) -> impl Iterator<Item = (i32, i32)> {
    assert!(target.x.0 <= target.x.1);
    assert!(target.y.0 <= target.y.1);

    // For simplicity
    assert!(target.x.0 > 0);
    assert!(target.x.1 > 0);
    assert!(target.y.0 < 0);
    assert!(target.y.1 < 0);

    let x_velocities = 0..=target.x.1;
    let y_velocities = target.y.0..=target.y.0.checked_abs().unwrap();

    iproduct!(x_velocities, y_velocities).filter_map(move |(vx, vy)| {
        let x_times = find_x_target_times(vx, target.x);
        let y_times = find_y_target_times(vy, target.y);

        if x_times.is_none() || y_times.is_none() {
            return None;
        }

        let x_times = x_times.unwrap();
        let y_times = y_times.unwrap();

        let valid = if x_times.0 <= y_times.0 {
            x_times.1 >= y_times.0
        } else {
            y_times.1 >= x_times.0
        };

        if valid {
            Some((vx, vy))
        } else {
            None
        }
    })
}

fn find_y_target_times(initial_velocity: i32, target: (i32, i32)) -> Option<(f64, f64)> {
    assert!(target.0 <= target.1);

    // For simplicity
    assert!(target.0 < 0);
    assert!(target.1 < 0);

    let times = find_arrival_times(initial_velocity, target.1).unwrap();
    let start_time = times.0.max(times.1);

    let times = find_arrival_times(initial_velocity, target.0).unwrap();
    let end_time = times.0.max(times.1);

    assert!(end_time >= start_time);

    let start_time = start_time.ceil();
    let end_time = end_time.floor();

    if end_time < start_time {
        None
    } else {
        Some((start_time, end_time))
    }
}

fn find_x_target_times(initial_velocity: i32, target: (i32, i32)) -> Option<(f64, f64)> {
    assert!(target.0 <= target.1);

    // For simplicity
    assert!(target.0 > 0);
    assert!(target.1 > 0);

    let start_time = if let Some(times) = find_arrival_times(initial_velocity, target.0) {
        times.0.min(times.1)
    } else {
        return None;
    };

    let end_time = if let Some(times) = find_arrival_times(initial_velocity, target.1) {
        times.0.min(times.1)
    } else {
        f64::INFINITY
    };

    assert!(end_time >= start_time);

    let start_time = start_time.ceil();
    let end_time = end_time.floor();

    if end_time < start_time {
        None
    } else {
        Some((start_time, end_time))
    }
}

fn find_arrival_times(initial_velocity: i32, target: i32) -> Option<(f64, f64)> {
    // https://www.symbolab.com/solver/step-by-step/solve%20for%20t%2C%20y%3D%20%5Cleft(2v-%5Cleft(t-1%5Cright)%5Cright)%20%5Cfrac%7Bt%7D%7B2%20%7D?or=input

    let v: f64 = initial_velocity.into();
    let x: f64 = target.into();

    let root = (4.0 * v * v + 4.0 * v + 1.0 - 8.0 * x).sqrt();
    if root.is_nan() {
        return None;
    }

    let t1 = (-2.0 * v - 1.0 + root) / -2.0;
    let t2 = (-2.0 * v - 1.0 - root) / -2.0;

    Some((t1, t2))
}

fn calculate_apogee(velocity: i32) -> i32 {
    if velocity < 0 {
        return 0;
    }

    ((velocity + 1) * velocity) / 2
}

fn parse_input() -> Result<TargetArea> {
    let mut input = String::new();
    input_file()?.read_to_string(&mut input)?;

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(?m)^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$"#).unwrap();
    }

    let captures = RE.captures(&input).context("Invalid input format")?;

    let x0: i32 = captures.get(1).unwrap().as_str().parse()?;
    let x1: i32 = captures.get(2).unwrap().as_str().parse()?;
    let y0: i32 = captures.get(3).unwrap().as_str().parse()?;
    let y1: i32 = captures.get(4).unwrap().as_str().parse()?;

    let result = TargetArea {
        x: (x0, x1),
        y: (y0, y1),
    };

    Ok(result)
}
