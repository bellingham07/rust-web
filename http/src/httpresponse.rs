use std::collections::HashMap;
use std::io;
use std::io::Write;
// 对于结构体而言，它的成员可以是自己持有的类型，比如String，也可以是引用类型，
// 而对于结构体，如果他的成员涉及到引用类型，此时，这个成员就需要指明生命周期

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "http/1.1".into(),
            status_code: "200".into(),
            status_text: "ok".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse) -> String {
        let res1 = response.clone();

        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &response.body.unwrap().len(),
            &res1.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
            Some(_h) => headers
        };
        response.status_text = match response.status_code {
            "200" => "ok".into(),
            "400" => "400 Bad Request".into(),
            "404" => "not found".into(),
            "500" => "500 Internal Server Error".into(),
            _ => "not found".into(),
        };

        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), io::Error> {
        let res = self.clone();
        let response_string = String::from(res);
        write!(write_stream, "{}", response_string).expect("write failed");
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut headers_string = "".into();
        for (k, v) in map.iter() {
            headers_string = format!("{}{}:{}\r\n", headers_string, k, v);
        }
        headers_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            None => "",
            Some(b) => b.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response = HttpResponse::new("200", None, Some("text".into()));
        let response_expected = HttpResponse {
            version: "http/1.1",
            status_code: "200",
            status_text: "ok",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("text".into()),
        };
        assert_eq!(response, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response = HttpResponse::new("404", None, Some("text".into()));
        let response_expected = HttpResponse {
            version: "http/1.1",
            status_code: "404",
            status_text: "not found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("text".into()),
        };
        assert_eq!(response, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "http/1.1",
            status_code: "404",
            status_text: "not found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("text".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string = "http/1.1 404 not found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\ntext";
        assert_eq!(http_string, actual_string);
    }
}