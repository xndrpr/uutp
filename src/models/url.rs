pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Url {
    pub fn parse(input: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = input.splitn(2, "://").collect();

        if parts.len() != 2 {
            return Err("Invalid URL format");
        }

        let scheme = parts[0].to_lowercase();
        let rest = parts[1];

        let (host_port, rest) = rest.split_once('/').unwrap_or((rest, ""));
        let (host, port) = if host_port.contains(':') {
            let (h, p) = host_port.split_once(':').unwrap();
            (
                h.to_string(),
                Some(p.parse::<u16>().map_err(|_| "Invalid port")?),
            )
        } else {
            (host_port.to_string(), None)
        };

        let (path_query_fragment, query_fragment) = rest.split_once('?').unwrap_or((rest, ""));
        let (path, fragment) = path_query_fragment
            .split_once('#')
            .unwrap_or((path_query_fragment, ""));
        let query = if !query_fragment.is_empty() {
            Some(query_fragment.to_string())
        } else {
            None
        };

        Ok(Self {
            scheme,
            host,
            port,
            path: path.to_string(),
            query,
            fragment: if !fragment.is_empty() {
                Some(fragment.to_string())
            } else {
                None
            },
        })
    }
}
