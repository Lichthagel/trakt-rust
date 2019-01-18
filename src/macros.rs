// A simple macro that creates the URL routes used to send requests
// TODO this needs to be improved
macro_rules! api_route {
    ($($r:expr),+) => {{
        let mut string = String::from("https://api.trakt.tv");
        $(
            string.push_str("/");
            string.push_str(&format!("{}", $r));
        )+
        string
    }};
}

macro_rules! api_pagination {
    ($url:expr, $page:expr, $limit:expr) => {
        format!("{}?page={}&limit={}", $url, $page, $limit)
    };
}

/// This macro creates requests for the Trakt API
macro_rules! api_request {
    ($client:expr, $client_id:expr, $route:expr) => {{
        match $client
            .get(&$route)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", $client_id)
            .send() {
            Ok(mut res) => {

                if res.status().is_success() {
                    let text = res.text().unwrap();
                    Ok(serde_json::from_str(text.as_str()).unwrap())
                } else {
                    Err(Error::RESPONSE(res))
                }
            },
            Err(e) => Err(Error::CONNECTION(e))
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn api_route_test() {
        assert_eq!(
            "https://api.trakt.tv/calendars/all/shows",
            api_route!("calendars/all/shows")
        )
    }
    #[test]
    fn multi_parameter_route_test() {
        assert_eq!(
            "https://api.trakt.tv/calendars/all/shows",
            api_route!("calendars", "all", "shows")
        )
    }
}
