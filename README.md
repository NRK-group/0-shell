
# Zero Shell

Zero Shell is a shell clone written in rust. If you have any suggestions to improve the project let me know

## Run Locally

Clone the project

```bash
  git clone git@github.com:NRK-group/0-shell.git
```

Go to the project directory

```bash
  cd ./shell
```

Start the application and install the dependecies

```bash
  cargo run
```

the result should be like:

```bash
➜  shell git:(main) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/0shell`
🛠️⚙️ 🤑 shell git:(main) ✗
```

## Features

- echo

The **echo** command is used to display a message or output data. It's commonly used to display strings directly on the terminal or to generate messages in scripts.

```bash
🛠️⚙️ 🤑 shell git:(main) ✗ echo "Hello"
Hello 
```

- cd

The **cd** command stands for "change directory." It's used to navigate between directories in the file system.

```bash
🛠️⚙️ 🤑 target git:(main) ✗ pwd
example/0-shell/shell/
🛠️⚙️ 🤑 shell git:(main) ✗ cd target
🛠️⚙️ 🤑 target git:(main) ✗ pwd
example/0-shell/shell/target
```

- ls

The **ls** command lists the contents of a directory.

```bash
🛠️⚙️ 🤑 shell git:(main) ✗ ls
Cargo.toml
target
Cargo.lock
README.md
src
```

- pwd

The pwd command, standing for "print working directory," displays the full path to the current directory.

```bash
🛠️⚙️ 🤑 shell git:(main) ✗ pwd
example/0-shell/shell
```

- cat

The cat command is used to display the contents of a file.

```bash
🛠️⚙️ 🤑 shell git:(main) ✗ cat Cargo.toml
[package]
name = "shell"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "0shell"

[dependencies]
ansi_term = "0.12.1"
rustyline = "11.0.0"
chrono = "0.4.19"
```

- cp

The **cp** command is used to copy files or directories from one location to another.

- rm

The **rm** command is used to remove files or directories.

- mv

The **mv** command stands for "move," and it's used to move or rename files and directories.

- mkdir

The **mkdir** command stands for "make directory," and it's used to create a new directory.

```bash
🛠️⚙️ 🤑 shell git:(main) ✗ ls
Cargo.toml
target
Cargo.lock
README.md
src
🛠️⚙️ 🤑 shell git:(main) ✗ mkdir hello
🛠️⚙️ 🤑 shell git:(main) ✗ ls
Cargo.toml
target
Cargo.lock
README.md
hello <- new file directory
src
```

- exit

The **exit** command is used to exit the current shell session.

- Ctrl + D

**Ctrl + D** is a keyboard shortcut used to send an End-of-File (EOF) character, which usually signals the end of input or terminates the current process in many shell environments.
