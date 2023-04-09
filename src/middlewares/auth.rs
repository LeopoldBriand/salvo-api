use jsonwebtoken::{DecodingKey, decode, Validation, Algorithm};
use salvo::prelude::*;
use once_cell::sync::OnceCell;

use crate::models::user::Claims;

static CONNECTED_USERS: OnceCell<Vec<String>> = OnceCell::new();

pub fn get_connected_users() -> &'static Vec<String> {
    CONNECTED_USERS.get().unwrap()
}

pub fn user_connection(uuid: String) {
    let mut users = get_connected_users().clone();
    users.push(uuid);
    CONNECTED_USERS.set(users).unwrap();
}

pub fn user_deconnection(uuid: String) {
    let mut users = get_connected_users().clone();
    let index = users.iter().position(|x| *x == uuid).unwrap();
    users.remove(index);
    CONNECTED_USERS.set(users).unwrap();
}


#[handler]
pub async fn auth_check(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env file.");
    match req.header::<String>("Authorisation") {
        Some(key) => { 
            let validation = Validation::new(Algorithm::HS256);
            let decode_result = decode::<Claims>(&key, &DecodingKey::from_secret(&secret_key.as_bytes()), &validation);
            match decode_result {
                Ok(claim) => { // Token is valid
                    let uuid: String = claim.claims.uuid;
                    let connected_users = get_connected_users();
                    if connected_users.contains(&uuid) { // User is still connected
                        depot.insert::<&str, String>("uuid", uuid);
                        ctrl.call_next(req, depot, res).await;
                    } else {
                        res.set_status_error(StatusError::expectation_failed());
                        res.render(Text::Plain("User is no longer connected".to_string()));
                        ctrl.skip_rest();
                    }
                },
                Err(error) => { // Token is invalid/expired
                    res.set_status_error(StatusError::unauthorized());
                    res.render(Text::Plain(error.to_string()));
                    ctrl.skip_rest();
                }
            }
            
        },
        None => { // Unauthorized because no token send
            res.set_status_error(StatusError::unauthorized());
            res.render(Text::Plain("No token send".to_string()));
            ctrl.skip_rest();
        }
    }
}