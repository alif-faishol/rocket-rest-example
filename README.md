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

## Example resources
Postman collection: [Here](https://www.getpostman.com/collections/753bba032e7cb704be69)
POST `/user/login`: to login.
POST `/user/sign_up`: to sign up.
GET `/user/current_user`: to get user details, jwt required.
GET `/user/logout`: jwt required, to add provided jwt to blacklist and clear expired jwts from blacklist. Blacklist stored in memory.
