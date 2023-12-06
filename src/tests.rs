use chrono::Utc;
use rand::{thread_rng, Rng};
use uuid::Uuid;

use super::*;

fn create_test_data(number_of_solves: usize) -> Vec<Solve> {
    let mut solves: Vec<Solve> = Vec::with_capacity(number_of_solves);
    let mut rng = thread_rng();
    for _ in 0..number_of_solves {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let time = Rng::gen_range(&mut rng, 8.0..19.0);
        solves.push(Solve {
            time,
            comment: String::from(""),
            scramble: String::from(""),
            id,
            created_at: now,
            plus_two: false,
            dnf: false,
        })
    }
    solves
}

#[test]
fn transforms_solves_correctly() {
    let old_solves = create_test_data(100);
    let new_solves = transform_solves(&old_solves);

    let mut ao5_dnf_count = 0;
    let mut ao5_none_count = 0;

    for i in new_solves.iter() {
        if i.dnf {
            ao5_dnf_count += 1;
        }
        if i.none {
            ao5_none_count += 1;
        }
    }

    assert_eq!(ao5_dnf_count, 0);
    assert_eq!(ao5_none_count, 4);
}

#[test]
fn sort_solves_by_latest() {
    let mut solves = transform_solves(&create_test_data(100));
    create_latest_solves(&mut solves);
    let mut prev = &solves[0];
    for i in solves.iter() {
        if prev.last_solve.created_at < i.last_solve.created_at {
            panic!("create_latest does not sort properly!");
        }
        prev = i;
    }
}

#[test]
fn sort_solves_by_best_ao5() {
    let mut solves = transform_solves(&create_test_data(100));
    create_best_ao5_solves(&mut solves);
    let mut prev = &solves[0];
    for i in solves.iter() {
        if i.time.is_nan() {
            continue;
        }
        if prev.time > i.time {
            panic!("create_best_ao5 does not sort properly!");
        }
        prev = i;
    }
}

#[test]
fn sort_solves_by_best_single() {
    let mut solves = transform_solves(&create_test_data(100));
    create_best_single_solves(&mut solves);
    let mut prev = &solves[0];
    for i in solves.iter() {
        if prev.last_solve.time > i.last_solve.time {
            panic!("create_best_single does not sort properly!");
        }
        prev = i;
    }
}

#[test]
fn sort_solves_correctly() {
    let old_solves = create_test_data(100);
    let new_solves = sort_solves(&old_solves, SortOrder::Best, SortBy::Single);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if prev.last_solve.time > i.last_solve.time {
            panic!("sort_solves does not sort properly! at SortOrder: Best and SortBy: Single");
        }
        prev = i;
    }

    let new_solves = sort_solves(&old_solves, SortOrder::Best, SortBy::Ao5);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if i.time.is_nan() {
            continue;
        }
        if prev.time > i.time {
            panic!("sort_solves does not sort properly! at SortOrder: Best and SortBy: Ao5");
        }
        prev = i;
    }

    let new_solves = sort_solves(&old_solves, SortOrder::Worst, SortBy::Single);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if prev.last_solve.time < i.last_solve.time {
            panic!("sort_solves does not sort properly! at SortOrder: Worst and SortBy: Single");
        }
        prev = i;
    }

    let new_solves = sort_solves(&old_solves, SortOrder::Worst, SortBy::Ao5);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if i.time.is_nan() {
            continue;
        }
        if prev.time < i.time {
            panic!("sort_solves does not sort properly! at SortOrder: Worst and SortBy: Ao5");
        }
        prev = i;
    }

    let new_solves = sort_solves(&old_solves, SortOrder::Latest, SortBy::Single);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if prev.last_solve.created_at < i.last_solve.created_at {
            panic!("sort_solves does not sort properly! at SortOrder: Latest and SortBy: Single");
        }
        prev = i;
    }

    let new_solves = sort_solves(&old_solves, SortOrder::Latest, SortBy::Ao5);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if prev.last_solve.created_at < i.last_solve.created_at {
            panic!("sort_solves does not sort properly! at SortOrder: Latest and SortBy: Ao5");
        }
        prev = i;
    }

    let new_solves = sort_solves(&old_solves, SortOrder::Oldest, SortBy::Single);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if prev.last_solve.created_at > i.last_solve.created_at {
            panic!("sort_solves does not sort properly! at SortOrder: Oldest and SortBy: Single");
        }
        prev = i;
    }

    let new_solves = sort_solves(&old_solves, SortOrder::Oldest, SortBy::Ao5);
    let mut prev = &new_solves[0];
    for i in new_solves.iter() {
        if prev.last_solve.created_at > i.last_solve.created_at {
            panic!("sort_solves does not sort properly! at SortOrder: Oldest and SortBy: Ao5");
        }
        prev = i;
    }
}
