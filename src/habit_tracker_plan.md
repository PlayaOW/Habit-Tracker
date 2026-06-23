# Habit Tracker CLI — Project Plan

## Phase 0: Setup
**Goal:** Working project skeleton, dependencies compile.

- [x] `cargo new habit_tracker`
- [x] Add all 5 dependencies to `Cargo.toml`
- [x] Create the 6 source files (empty `mod` declarations in `main.rs`)
- [x] Confirm `cargo build` succeeds with no code yet

**Checkpoint:** `cargo build` passes.

---

## Phase 1: Models
**Goal:** All data types defined, no logic yet.

- [x] Define `Status` enum with 2 variants
- [x] Define `Category` enum with fixed variants + `Other(String)`
- [x] Define `Occurrence` enum — `Daily`, `Weekly(Vec<Weekday>)`, `Monthly(Vec<u8>)`
- [x] Define `Habit` struct with all fields
- [x] Define `User` struct — includes `Vec<Habit>` and `next_habit_id: u64`
- [x] Define `AppData` struct — just `Vec<User>`
- [x] Add `#[derive(Debug, Serialize, Deserialize)]` to everything
- [x] Implement `Display` for `Status`, `Category`, `Occurrence`, `Habit`

**Checkpoint:** `cargo build` passes. You can construct a `Habit` in `main.rs` and `println!` it.

---

## Phase 2: Storage
**Goal:** Persist and reload `AppData` from disk.

- [x] Decide on file path — `~/.habit_tracker/data.json`
- [x] Write `fn load() -> AppData` — reads JSON from disk, returns `AppData::default()` if file doesn't exist
- [x] Write `fn save(data: &AppData)` — serializes to JSON, writes to disk
- [ ] Handle `create_dir_all` so the directory is created if missing
- [x] Test in `main.rs`: construct a dummy `AppData`, save it, reload it, print it

**Checkpoint:** You can kill the program and relaunch — data survives.

---

## Phase 3: Auth
**Goal:** Register and login against stored hashed passwords.

- [ ] Write `fn hash_password(password: &str) -> String` using `sha2` + `hex`
- [ ] Write `fn register(data: &mut AppData)` — prompt username/age/password, reject duplicates, push new `User`
- [ ] Write `fn login(data: &mut AppData) -> Option<&mut User>` — prompt credentials, hash input, compare to stored hash
- [ ] Never store or print plaintext passwords anywhere

**Checkpoint:** Register a user, exit, relaunch, login successfully. Wrong password is rejected.

---

## Phase 4: Habit Management
**Goal:** Full CRUD on habits for a logged-in user.

- [ ] Write `fn add_habit(user: &mut User)` — prompt all fields, parse `Occurrence` and `NaiveTime` from user input, push to `user.habits`, increment `next_habit_id`
- [x] Write `fn list_habits(user: &User)` — print all habits with their current status, call `refresh_for_today()` on each first
- [ ] Write `fn complete_habit(user: &mut User)` — prompt habit id, mark complete
- [ ] Write `fn skip_habit(user: &mut User)` — prompt habit id, mark skipped
- [ ] Write `fn delete_habit(user: &mut User)` — prompt habit id, remove from vec

**Checkpoint:** Full round-trip — add habits, exit, relaunch, habits are still there, can be checked off.

---

## Phase 5: Main Loop
**Goal:** Cohesive CLI experience from launch to exit.

- [ ] On launch: print welcome, prompt login or register
- [ ] After auth: show main menu in a loop
- [ ] Menu options: List Habits, Add Habit, Complete Habit, Skip Habit, Delete Habit, Logout, Quit
- [ ] `save()` after every mutation (add, complete, skip, delete)
- [ ] Logout returns to the login/register prompt
- [ ] Quit exits the process cleanly

**Checkpoint:** Full end-to-end usable app. Show it to someone and they can use it without explanation.

---

## Phase 6: Notifications (Stretch Goal)
**Goal:** Terminal reminder when a habit's scheduled time arrives.

- [ ] Wrap `AppData` in `Arc<Mutex<AppData>>` — read about these before touching them
- [ ] Spawn a background thread with `std::thread::spawn`
- [ ] Thread loop: sleep 60 seconds, lock the mutex, check each habit's `scheduled_time` against `Local::now().time()`, print a reminder if within the same minute and status is still `Pending`
- [ ] Print the terminal bell character `\x07` alongside the reminder for an audible ping

**Checkpoint:** Leave the app idle — reminder prints at the right time.

---

## Concepts You'll Learn at Each Phase

| Phase | Rust Concepts |
|---|---|
| 1 | Enums with data, structs, `derive`, `Display` trait |
| 2 | File I/O, `Result`, `?` operator, `serde` serialization |
| 3 | External crates (`sha2`), string manipulation, `Option` |
| 4 | `&mut` references, `Vec` methods, `match` on user input, parsing |
| 5 | Loops, ownership across function calls, program flow |
| 6 | `Arc`, `Mutex`, threads, shared state |

---
