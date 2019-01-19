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

macro_rules! api_parameter {
    ($url:expr, $(($a:expr, $b:expr)),+) => {{
        let mut string: String = $url;
        $(
            string.push_str(&format!("?{}={}", $a, $b));
        )+
        string
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
    #[test]
    fn api_parameter_test() {
        assert_eq!(
            "https://api.trakt.tv/example?test=1",
            api_parameter!(String::from("https://api.trakt.tv/example"), ("test", "1"))
        )
    }
}
