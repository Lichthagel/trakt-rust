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
