# Rocket JSON Rest API example
This repository is an example implementation
of Rocket.rs as a JSON Rest API service. Created to be used
as a starter code for myself.

I'm just started using rust, any feedback to improve the code would be apreciated.

## How to run

  - Make sure Rust is installed. [Here](https://www.rust-lang.org/tools/install).
  - Use nightly build of rust.
      ```
      rustup override set nightly
      ```
  - Install diesel cli
      ```
      cargo install diesel_cli
      ```
  - Create .env
      ```
      cp .env.example .env
      ```
  - Edit .env to use your MySQL database, or use provided
    dockerized MySQL server.
  - Run migrations
      ```
      diesel migration run
      ```
  - Run
      ```
      cargo run
      ```
