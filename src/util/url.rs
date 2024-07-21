use crate::client::endpoint::EndpointBuilder;

pub fn build_subreddit(name: &str) -> EndpointBuilder {
    EndpointBuilder::new(format!("r/{name}/"))
}
