# dewr - module 02

Data Engineering with Rust - module 02

  | module 02 ||
  |:--|:--|
  | file version  | 0.1.0 |
  | file created  | 2025/02/11 |
  | last change   | 2025/02/25 |

## check rust and cargo version

  - ```cargo --version```
  - ```rustc --version```

## setup
   there is a tool to copy the template projects and do the necessary replacements
   ```
   tools/new_project.sh <new_project>
   ```
   <new_project> must be the project name used in Cargo.toml, without space etc. The project name
                 is then derived by replacing "_" by " "
   
   Note: clap has to be added using ```cargo add clap --features derive```

## compile and run the template
  ```
  cargo build
  cargo run
  # build/run release
  cargo run --release
  # or directly
  ./target/debug/rust-template 
  # to get stacktraces, run
  RUST_BACKTRACE=1 cargo run
  # or
  RUST_BACKTRACE=full cargo run
  ```

## project links

  | project liks ||
  |:--|:--|
  | rust | https://www.rust-lang.org/ |
