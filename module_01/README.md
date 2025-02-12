# dewr - module xx

Data Engineering with Rust - module xx

  | module xx ||
  |:--|:--|
  | file version  | 0.1.0 |
  | file created  | 2025/02/12 |
  | last change   | 2025/02/12 |

## check rust and cargo version

  - ```cargo --version```
  - ```rustc --version```

## setup
   ```
   cp -a module_xx module_xx
   cd module_xx
   sed -i -e 's/lab\_template/module_xx/g' -e 's/lab\ template/module xx/g' Cargo.toml
   sed -i -e 's/lab\_template/module_xx/g' -e 's/lab\ template/module xx/g' Makefile
   sed -i -e 's/lab\_template/module_xx/g' -e 's/lab\ template/module xx/g' README.md
   sed -i -e 's/lab\_template/module_xx/g' -e 's/lab\ template/module xx/g' README.src/main.rs
   ```
   Note: do not remove the backslashes above, otherwise the commands will also be 
         replaced and rendered wrong.

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
