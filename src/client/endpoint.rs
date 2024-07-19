#[derive(Debug)]
pub struct EndpointBuilder {
    pub path: String,
    pub query: Vec<(String, String)>,
    pub with_dot_json: bool,
}

impl EndpointBuilder {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            query: Vec::new(),
            with_dot_json: true,
        }
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.with_query(key, value);
        self
    }

    pub fn with_query(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn build(self, base_url: &str) -> String {
        let dot_json = if self.with_dot_json { ".json" } else { "" };
        let mut joined = if self.path.len() == 0 || self.path.starts_with('/') {
            format!("{base_url}{}/{dot_json}", self.path)
        } else {
            format!("{base_url}/{}/{dot_json}", self.path)
        };

        if self.query.len() > 0 {
            joined.push('?');
        }

        for (key, value) in self.query {
            joined.push_str(&key);
            joined.push('=');
            joined.push_str(&value);
            joined.push('&');
        }
        joined
    }
}

impl<'a> From<&'a str> for EndpointBuilder {
    fn from(value: &'a str) -> Self {
        EndpointBuilder::new(value)
    }
}

impl From<String> for EndpointBuilder {
    fn from(value: String) -> Self {
        EndpointBuilder::new(value)
    }
}
