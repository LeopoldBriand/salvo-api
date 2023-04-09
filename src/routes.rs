use salvo::prelude::*;
use crate::{controllers::user::{login, logout, signin, get_all_users, get_user, get_connected}, middlewares::auth::{auth_check}};

pub fn build_routes() -> Router {
    Router::new()
        .get(ping)
        // Auth routes
        .push(Router::with_path("login").post(login))
        .push(Router::with_path("logout").hoop(auth_check).post(logout))
        .push(Router::with_path("signin").post(signin))
        // User routes
        .push(Router::with_path("user").hoop(auth_check).get(get_all_users))
        .push(Router::with_path("user/connected").hoop(auth_check).get(get_connected))
        .push(Router::with_path("user/<id>").hoop(auth_check).get(get_user))
}

#[handler]
pub async fn ping(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render("APi is alived");
}