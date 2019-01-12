// A simple macro that creates the URL routes used to send requests
macro_rules! api_route {
    ($r:expr) => {{
        concat!("https://api.trakt.tv/", $r)
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
}
