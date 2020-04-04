
# Learning Log (newest first)

## 4/3/2020

### Goal for next time

Spend more time coding in Rust. Come back to learning GitHub and Markdown after a few good programming sessions.

### Rust/Cargo

* Cargo.toml is called the *manifest* and requires a specific format

* Crates only need one `Cargo.toml` file which is located in root folder

* [[bin]] is a *target table* section in the manifest, which tells cargo to select between binaries with `cargo run --bin <some_binary>`

* The `default-run = <name>` is a field under `[package]`, and specifies which binary will be chosen by `cargo run`

### Git(Hub)

* **Projects** offers Kanban boards, and **Actions** allows automated testing

* `git add <filename>` must be used to add files to staging prior to commit

* `.gitignore` files can be used to tell git to ignore a file...obviously

* `git status` prints out which, of the modified files, have been added to staging and which have not

* `git commit` to finalize, and `git push` to update repository on GitHub
