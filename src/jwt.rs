extern crate chrono;
extern crate jsonwebtoken;

use chrono::{Utc, Duration};
use crate::InternalState;
use jsonwebtoken::{encode, decode, Algorithm, Validation, Header};
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct JwtPayload {
    pub sub: u64,
    pub exp: i64
}

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    pub token: String,
    pub claims: JwtPayload,
}

pub fn create_token(id: u64) -> String {
    let exp = Utc::now() + Duration::seconds(60 * 60); // an hour
    let claim = JwtPayload {
        sub: id,
        exp: exp.timestamp_millis()
    };

    let secret = env::var("JWT_SECRET");
    let secret = secret.as_ref().map(String::as_str).unwrap_or("secret");

    "Bearer ".to_string() + &encode(&Header::new(Algorithm::HS256), &claim, secret.as_ref()).unwrap()
}

impl<'a, 'r> FromRequest<'a, 'r> for Jwt {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Jwt, ()> {
        let token: Vec<_> = request.headers().get("Authorization").collect();
        let internal_state = request.guard::<State<InternalState>>()?;
        let revoked_tokens = internal_state.revoked_tokens.lock().unwrap();

        let secret = env::var("JWT_SECRET");
        let secret = secret.as_ref().map(String::as_str).unwrap_or("secret");

        if token.len() < 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        // Removes "Bearer " part
        let token = &token[0][7..];
        if revoked_tokens.contains_key(token) {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let claim = decode::<JwtPayload>(
            &token,
            secret.as_ref(),
            &Validation::new(Algorithm::HS256)
        );

        match claim {
            Ok(c) => {
                if c.claims.exp > Utc::now().timestamp_millis() {
                    let token: Jwt = Jwt {
                        token: token.to_string(),
                        claims: c.claims
                    };
                    Outcome::Success(token)
                } else {
                    Outcome::Failure((Status::Unauthorized, ()))
                }
            },
            Err(err) => {
                eprintln!("{}", err.to_string());
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}
