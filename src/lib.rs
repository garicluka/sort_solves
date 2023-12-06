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

pub fn transform_solves(old_solves: &Vec<Solve>) -> Vec<Ao5Solve> {
    let mut new_solves = Vec::new();

    for i in 0..old_solves.len() {
        if i > old_solves.len() - 5 {
            let mut time = old_solves[i].time;
            if old_solves[i].plus_two {
                time += 2.0;
            }
            new_solves.push(Ao5Solve {
                time: f64::NAN,
                none: true,
                dnf: false,
                last_solve: Solve {
                    time,
                    ..old_solves[i].clone()
                },
            });
            continue;
        }

        let mut valid_times = 0;
        let mut best_time = f64::INFINITY;
        let mut worst_time = f64::NEG_INFINITY;
        let mut time_sum = 0.0;
        let mut times = [
            old_solves[i].time,
            old_solves[i + 1].time,
            old_solves[i + 2].time,
            old_solves[i + 3].time,
            old_solves[i + 4].time,
        ];
        for j in i..(i + 5) {
            if old_solves[j].plus_two {
                times[j - i] += 2.0;
            }
            if !old_solves[j].dnf {
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

        new_solves.push(Ao5Solve {
            time,
            dnf,
            none: false,
            last_solve: Solve {
                ..old_solves[i].clone()
            },
        })
    }

    new_solves
}

pub fn create_latest_solves(old_solves: &mut Vec<Ao5Solve>) {
    old_solves.sort_by(|a, b| {
        if a.last_solve.created_at > b.last_solve.created_at {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });
}

pub fn sort_solves(
    old_solves: &Vec<Solve>,
    sort_order: SortOrder,
    sort_by: SortBy,
) -> Vec<Ao5Solve> {
    let mut new_solves = transform_solves(old_solves);

    match sort_order {
        SortOrder::Latest => {
            create_latest_solves(&mut new_solves);
        }
        SortOrder::Oldest => {
            create_latest_solves(&mut new_solves);
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
