/// The ID of a flair. This should be a GUID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FlairId(String);

impl std::ops::Deref for FlairId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
