use std::collections::HashSet;

use mock_api::{
    database::{
        subreddit::{SubredditData, SubredditMeta},
        thing::ThingId,
        user::{UserData, UserMeta},
    },
    MockApi,
};
use roux::{client::RedditClient, Config};

#[tokio::test]
pub async fn get_mock_subreddit() {
    let mock = MockApi::new();

    let sub = SubredditData {
        id: ThingId::new(),
        display_name: "roux".into(),
    };

    mock.db().insert_subreddit(SubredditMeta {
        data: sub.clone(),
        moderators: HashSet::new(),
    });

    let server = mock.start();
    let client = roux::client::UnauthedClient::new()
        .unwrap()
        .with_base_url(server.url());

    let subclient = client.subreddit(&sub.display_name);
    let info = subclient.about().await.unwrap();

    assert_eq!(info.id, Some(sub.id.to_string()));
    assert_eq!(info.display_name, Some(sub.display_name));
    assert_eq!(info.name, Some(format!("t5_{}", sub.id)));
}

#[tokio::test]
pub async fn mock_login() {
    let mock = MockApi::new();

    let sub = SubredditData {
        id: ThingId::new(),
        display_name: "roux".into(),
    };
    let user = UserData {
        id: ThingId::new(),
        username: "user1".into(),
    };

    mock.db().insert_subreddit(SubredditMeta {
        data: sub.clone(),
        moderators: HashSet::new(),
    });
    mock.db().insert_user(UserMeta {
        access_token: format!("#token_{}", user.id),
        password: "pass1".into(),
        data: user.clone(),
    });

    let info = mock.start();

    let client = roux::client::OAuthClient::new(
        Config::new("user_agent", "client_id", "client_secret")
            .base_url(info.url())
            .username("user1")
            .password("pass1"),
    )
    .unwrap()
    .login()
    .await
    .unwrap();

    let subclient = client.subreddit("name");

    let mods = subclient.moderators().await.unwrap();
    println!("{mods:#?}");
}
