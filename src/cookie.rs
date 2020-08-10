use actix_identity::Identity;
use epitok::auth::Auth;

pub fn set(id: Identity, auth: Auth) -> bool {
    let autologin = match auth.autologin() {
        Some(autologin) => autologin,
        None => return false,
    };
    let login = match auth.login() {
        Some(login) => login,
        None => return false,
    };

    id.remember(format!("{}!#{}", login, autologin));
    true
}

fn get_index(cookie: &str, i: usize) -> &str {
    let contents: Vec<&str> = cookie.split("!#").collect();
    contents[i]
}

pub fn get_login(cookie: &str) -> &str {
    get_index(cookie, 0)
}

pub fn get_autologin(cookie: &str) -> &str {
    get_index(cookie, 1)
}
