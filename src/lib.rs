pub mod types;

use std::{cmp::Ordering, process::ExitCode};
use types::*;

pub fn run() -> Result<(), ExitCode> {
    let args = Cli::parse();
    let raw_solves = std::fs::read_to_string(args.from).map_err(|e| {
        eprintln!("ERROR: failed reading solves, {}", e);
        1
    })?;
    let solves: Vec<Solve> = serde_json::from_str(&raw_solves).map_err(|e| {
        eprintln!("ERROR: failed deserializing solves, {}", e);
        1
    })?;

    let sorted_solves = sort_solves(&solves, args.sort_order, args.sort_by);

    let serialized_sorted_solves = serde_json::to_string(&sorted_solves).map_err(|e| {
        eprintln!("ERROR: failed serializing solves, {}", e);
        1
    })?;
    std::fs::write(args.to, serialized_sorted_solves).map_err(|e| {
        eprintln!("ERROR: failed writing solves, {}", e);
        1
    })?;

    Ok(())
}

pub fn create_best_ao5_solves(solves: &mut Vec<Ao5Solve>) {
    solves.sort_by(|a, b| {
        if a.none {
            return Ordering::Greater;
        }
        if b.none {
            return Ordering::Less;
        }
        if a.dnf {
            return Ordering::Greater;
        }
        if b.dnf {
            return Ordering::Less;
        }
        if a.time >= b.time {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    });
}

pub fn create_best_single_solves(solves: &mut Vec<Ao5Solve>) {
    solves.sort_by(|a, b| {
        if a.last_solve.dnf {
            return Ordering::Greater;
        }
        if b.last_solve.dnf {
            return Ordering::Less;
        }
        if a.last_solve.time >= b.last_solve.time {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    });
}

pub fn transform_and_sort_by_latest(old_solves: &Vec<Solve>) -> Vec<Ao5Solve> {
    let mut new_solves: Vec<Ao5Solve> = old_solves
        .iter()
        .map(|solve| {
            return Ao5Solve {
                time: f64::NAN,
                dnf: false,
                none: false,
                last_solve: solve.clone(),
            };
        })
        .collect();

    new_solves.sort_by(|a, b| {
        if a.last_solve.created_at > b.last_solve.created_at {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    for i in 0..new_solves.len() {
        if i > old_solves.len() - 5 {
            let mut time = new_solves[i].last_solve.time;
            if new_solves[i].last_solve.plus_two {
                time += 2.0;
            }
            new_solves[i].time = f64::NAN;
            new_solves[i].none = true;
            new_solves[i].dnf = false;
            new_solves[i].last_solve.time = time;
            continue;
        }

        let mut valid_times = 0;
        let mut best_time = f64::INFINITY;
        let mut worst_time = f64::NEG_INFINITY;
        let mut time_sum = 0.0;
        let mut times = [
            new_solves[i].last_solve.time,
            new_solves[i + 1].last_solve.time,
            new_solves[i + 2].last_solve.time,
            new_solves[i + 3].last_solve.time,
            new_solves[i + 4].last_solve.time,
        ];
        for j in i..(i + 5) {
            if new_solves[j].last_solve.plus_two {
                times[j - i] += 2.0;
            }
            if !new_solves[j].last_solve.dnf {
                valid_times += 1;
                if times[j - i] < best_time {
                    best_time = times[j - i];
                }
                if times[j - i] > worst_time {
                    worst_time = times[j - i];
                }
                time_sum += times[j - i];
            }
        }

        let time;
        let mut dnf = false;
        if valid_times < 4 {
            time = f64::NAN;
            dnf = true;
        } else if valid_times == 4 {
            time = (time_sum - best_time) / 3.0;
        } else {
            time = (time_sum - best_time - worst_time) / 3.0;
        }
        new_solves[i].time = time;
        new_solves[i].none = false;
        new_solves[i].dnf = dnf;
        new_solves[i].last_solve.time = times[0];
    }

    new_solves
}

pub fn sort_solves(
    old_solves: &Vec<Solve>,
    sort_order: SortOrder,
    sort_by: SortBy,
) -> Vec<Ao5Solve> {
    let mut new_solves = transform_and_sort_by_latest(old_solves);

    match sort_order {
        SortOrder::Latest => {
            // already sorted by latest
        }
        SortOrder::Oldest => {
            new_solves.reverse();
        }
        SortOrder::Best => match sort_by {
            SortBy::Single => {
                create_best_single_solves(&mut new_solves);
            }
            SortBy::Ao5 => {
                create_best_ao5_solves(&mut new_solves);
            }
        },
        SortOrder::Worst => match sort_by {
            SortBy::Single => {
                create_best_single_solves(&mut new_solves);
                new_solves.reverse();
            }
            SortBy::Ao5 => {
                create_best_ao5_solves(&mut new_solves);
                new_solves.reverse();
            }
        },
    };

    new_solves
}

#[cfg(test)]
mod tests;
