fn cookie_get_index(cookie: &str, i: usize) -> &str {
    let contents: Vec<&str> = cookie.split("!#").collect();
    contents[i]
}

pub fn cookie_get_login(cookie: &str) -> &str {
    cookie_get_index(cookie, 0)
}

pub fn cookie_get_autologin(cookie: &str) -> &str {
    cookie_get_index(cookie, 1)
}
