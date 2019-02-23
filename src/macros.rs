// A simple macro that creates the URL routes used to send requests
// TODO this needs to be improved
macro_rules! api_url {
    (($($y:expr),+) $(,($a:expr, $b:expr))*) => {{
        let mut string: String = String::new();
        $(
            string.push_str("/");
            string.push_str(&format!("{}", $y));
        )+

        let mut _first: bool = true;

        $(
            if _first {
                _first = false;
                string.push_str(&format!("?{}={}", $a, $b));
            } else {
                string.push_str(&format!("&{}={}", $a, $b));
            }

        )*
        string
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn combined_force_test() {
        assert_eq!(
            "/test/1/2/3?test=1&test1=2",
            &api_url!(("test", "1", "2", "3"), ("test", "1"), ("test1", "2"))
        )
    }

    #[test]
    fn combined_force_test_only_url() {
        assert_eq!("/test/1/2/3", &api_url!(("test", "1", "2", "3")))
    }
}
