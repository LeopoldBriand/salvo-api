use jsonwebtoken::{encode, Header, EncodingKey};
use salvo::http::cookie::time::{OffsetDateTime, Duration};
use salvo::prelude::*;
use uuid::Uuid;

use crate::middlewares::auth::{user_connection, user_deconnection, get_connected_users};
use crate::models::user::{User, SigninUserForm, LoginUserForm, Claims};
use crate::db::get_db;

#[handler]
pub async fn login(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env file.");
    let login_user_form: LoginUserForm = req.parse_json::<LoginUserForm>().await.unwrap();
    let data_result = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(get_db())
        .await
        .unwrap();
    let data = data_result
        .iter()
        .find(|u| u.username == login_user_form.username && u.password == login_user_form.password);
    match data {
        Some(user) => { // User exist in database
            let claim = Claims{uuid: user.id.to_string(), exp: (OffsetDateTime::now_utc() + Duration::days(14)).unix_timestamp()};
            let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(&secret_key.as_bytes())).unwrap();
            user_connection(user.id.to_string());
            res.set_status_code(StatusCode::OK);
            res.render(Text::Plain(token));
        },
        None => { // User doesn't exist in database
            res.set_status_error(StatusError::not_acceptable());
            res.render(Text::Plain("Invalid username/password".to_string()));
        }
    }
}

#[handler]
pub async fn logout(_req: &mut Request, depot: &mut Depot, _res: &mut Response) {
    let id: String = depot.get::<String>("uuid").unwrap().to_string();
    user_deconnection(id.to_string());
}

#[handler]
pub async fn get_connected(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let connected_users = get_connected_users();
    res.render(serde_json::to_string(&connected_users).unwrap());

}

#[handler]
pub async fn signin(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env file.");
    let signin_user_form: SigninUserForm = req.parse_json::<SigninUserForm>().await.unwrap();
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, username, password) VALUES (?, ?, ?)")
        .bind(id.to_string())
        .bind(signin_user_form.username)
        .bind(signin_user_form.password)
        .execute(get_db())
        .await
        .unwrap();
    let claim = Claims{uuid: id.to_string(), exp: (OffsetDateTime::now_utc() + Duration::days(14)).unix_timestamp()};
    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(&secret_key.as_bytes())).unwrap();
    user_connection(id.to_string());
    res.set_status_code(StatusCode::OK);
    res.render(Text::Plain(token));
}

#[handler]
pub async fn get_all_users(res: &mut Response) {
    let data = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(get_db())
        .await
        .unwrap();
    res.render(serde_json::to_string(&data).unwrap());
}

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let id = req.params().get::<str>("id").unwrap();
    let data: User = sqlx::query_as::<_, User>("select * from users where id = ?")
        .bind(id)
        .fetch_one(get_db())
        .await
        .unwrap();
    res.render(serde_json::to_string(&data).unwrap());
}

