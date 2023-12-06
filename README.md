sort_solves
------------

sort_solves is binary that takes json file with rubik's cube solves and makes new json file with sorted solves.

[![thing](https://img.shields.io/badge/v0.2.0%20-%20sort_solves?label=crates.io)](https://www.crates.io/crates/sort_solves)



```sh
$ sort_solves best ao5 ./solves.json ./sorted-solves.json
```

solves.json example:
```json
[{"time":12.093625067611864,"comment":"","scramble":"LRLRLRLRLRRLRL",
"id":"13ced05e-612b-4eb0-a55e-1e551f5f6093",
"created_at":"2023-12-06T07:04:04.464338841Z","plus_two":false,"dnf":false}]
```
rust solve type (needs chrono::serde feature for date to automatically serialize and deserialize)
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Solve {
    pub time: f64,
    pub comment: String,
    pub scramble: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub plus_two: bool,
    pub dnf: bool,
}
```

sorted-solves.json example:
```json
[{"last_solve":{"time":9.465833877444984,"comment":"",
"scramble":"LRLRLRLRLRRLRL","id":"f9c2b326-0f64-4964-9616-4557656ac331",
"created_at":"2023-12-06T07:04:04.464219529Z","plus_two":false,"dnf":false},
"time":9.93495505183404,"none":false,"dnf":false}]
```
rust sorted-solve type (where Solve is the one previously shown)
```rust
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Ao5Solve {
    pub last_solve: Solve,
    pub time: f64,
    pub none: bool,
    pub dnf: bool,
}

```
this project is under MIT license
