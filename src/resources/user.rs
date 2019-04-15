extern crate bcrypt;

use chrono::{Utc};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::Route;
use rocket::State;
use rocket_contrib::json::*;

use crate::InternalState;
use crate::schema::users;
use crate::MainDB;
use crate::ResponseJson;
use crate::jwt::{create_token, Jwt};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
struct NewUser {
    full_name: String,
    email: String,
    password: String,
}

#[derive(Queryable, Serialize)]
struct User {
    id: u64,
    email: String,
    full_name: String,
    password: String,
}

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String
}

#[post("/sign_up", format = "json", data = "<data>")]
fn sign_up(connection: MainDB, data: Json<NewUser>) -> Result<ResponseJson, Status> {
    let connection: &diesel::MysqlConnection = &connection;

    let mut data: NewUser = data.into_inner();
    match bcrypt::hash(data.password, bcrypt::DEFAULT_COST) {
        Ok(hashed_password) => {
            data.password = hashed_password;
        },
        Err(_) => return Err(Status::InternalServerError)
    }
    use crate::schema::users::dsl::*;
    use diesel::insert_into;
    match insert_into(users)
        .values(&data)
        .execute(connection) {
            Ok(_rows_inserted) => {
                match users
                    .filter(email.eq(data.email))
                    .load::<User>(connection) {
                        Ok(result) => {
                            match result.first() {
                                Some(row) => Ok(ResponseJson {
                                    data: json!({
                                        "full_name": row.full_name,
                                        "email": row.email
                                    }),
                                    status: Status::Ok
                                }),
                                None => Err(Status::InternalServerError)
                            }
                        },
                        Err(_err) => Err(Status::InternalServerError)
                    }
            },
            Err(err) => {
                if err.to_string()[0..15] == *"Duplicate entry" {
                    Ok(ResponseJson {
                        data: json!({
                            "message": "User already exist!"
                        }),
                        status: Status::Conflict
                    })
                } else {
                    Err(Status::InternalServerError)
                }
            }
        }
}

#[post("/login", format = "json", data = "<data>")]
fn login(connection: MainDB, data: Json<Login>) -> Result<ResponseJson, Status> {
    use crate::schema::users::dsl::*;

    let connection: &diesel::MysqlConnection = &connection;
    let data: Login = data.into_inner();

    let result = match users
        .filter(email.eq(data.email))
        .load::<User>(connection) {
            Ok(r) => r,
            Err(_) => return Err(Status::InternalServerError)
        };

    let user = match result.first() {
        Some(user) => user,
        None => return Ok(ResponseJson {
            data: json!({ "message": "User not found!" }),
            status: Status::Forbidden
        })
    };

    match bcrypt::verify(data.password, &user.password) {
        Ok(verified) => {
            if verified {
                let token = create_token(user.id);
                Ok(ResponseJson {
                    data: json!({
                        "message": "Login success!",
                        "token": token
                    }),
                    status: Status::Ok
                })
            } else {
                Ok(ResponseJson {
                    data: json!({ "message": "Wrong password!" }),
                    status: Status::Forbidden
                })
            }
        },
        Err(_) => return Err(Status::InternalServerError)
    }
}

#[get("/current_user", format = "json")]
fn current_user(connection: MainDB, jwt: Jwt) -> Result<ResponseJson, Status> {
    use crate::schema::users::dsl::*;
    let connection: &diesel::MysqlConnection = &connection;

    let result = match users.filter(id.eq(jwt.claims.sub)).load::<User>(connection) {
        Ok(result) => result,
        Err(_) => return Err(Status::InternalServerError)
    };

    match result.first() {
        Some(user) => Ok(ResponseJson {
            data: json!({
                "full_name": user.full_name,
                "email": user.email
            }),
            status: Status::Ok
        }),
        None => Err(Status::Unauthorized)
    }
}

#[get("/logout")]
fn logout(jwt: Jwt, internal_state: State<InternalState>) -> Result<ResponseJson, Status> {
    let mut revoked_tokens = internal_state.revoked_tokens.lock().unwrap();

    revoked_tokens.insert(
        (*jwt.token).to_string(),
        jwt.claims.exp
    );
    revoked_tokens.retain(|_, exp| *exp > Utc::now().timestamp_millis());

    Ok(ResponseJson {
        data: json!({ "token": jwt.token }),
        status: Status::Ok
    })
}

pub fn routes() -> Vec<Route> {
    routes![sign_up, login, current_user, logout]
}
