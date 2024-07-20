use std::borrow::Cow;

use serde::Serialize;

pub struct FormBuilder<'a> {
    values: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> FormBuilder<'a> {
    pub fn new() -> Self {
        Self {
            values: vec![(Cow::Borrowed("api_type"), Cow::Borrowed("json"))],
        }
    }

    pub fn with(mut self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        self.add(key, value);
        self
    }

    pub fn add(&mut self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) {
        self.values.push((key.into(), value.into()));
    }
}

impl<'form> Serialize for FormBuilder<'form> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.values.serialize(serializer)
    }
}

impl<'a, const N: usize> From<[(&'static str, &'a str); N]> for FormBuilder<'a> {
    fn from(value: [(&'static str, &'a str); N]) -> Self {
        let mut form = FormBuilder::new();
        for (key, value) in value {
            form.add(key, value);
        }
        form
    }
}

#[cfg(test)]
mod tests {
    use super::FormBuilder;

    #[test]
    pub fn test_url_encoded() {
        let form = FormBuilder::new()
            .with("text", "goeshere")
            .with("another-value", format!("somemore stuff"));

        let encoded = serde_urlencoded::to_string(&form).unwrap();

        assert_eq!(
            encoded,
            r#"api_type=json&text=goeshere&another-value=somemore+stuff"#
        );
    }
}
