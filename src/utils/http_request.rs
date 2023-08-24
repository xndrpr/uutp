use crate::models::http_method::HttpMethod;

#[allow(dead_code)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub host: String,
    pub path: String,
    pub conn: String,
}

impl Default for HttpRequest {
    fn default() -> Self {
        HttpRequest {
            method: HttpMethod::GET,
            host: String::new(),
            conn: String::new(),
            path: String::new(),
        }
    }
}

#[allow(dead_code)]
impl HttpRequest {
    pub fn build(&self) -> String {
        let method_str = match self.method {
            HttpMethod::GET => "GET",
        };

        format!(
            "{} /{} HTTP/1.1\r\nHost: {}\r\nAccept: text/html\r\nReferrer: {}\r\nUser-Agent: Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/W.X.Y.Z Safari/537.36\r\nConnection: {}\r\n\r\n",
            method_str, self.path, self.host, self.host, self.conn
        )
    }
}
