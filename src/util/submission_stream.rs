use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
};

use crate::{
    client::{AuthedClient, OAuthClient, RedditClient, UnauthedClient},
    util::{FeedOption, RouxError},
};

/// A helper struct to help manage requesting submissions from multiple subreddits,
/// automatically handling requesting a multi-reddit (e.g. `/r/sub1+sub2/new`),
/// ~~performing catch-ups if posts are missed, or falling back to single-sub requests
/// if there are too many catch-ups occurring~~ (TODO: catchups).
pub struct SubmissionStream<T> {
    batch_size: usize,
    subreddits: HashMap<String, SubredditData>,
    _marker: PhantomData<T>,
}

impl<T> SubmissionStream<T> {
    /// Initializes a stream of subreddits from the provided subreddits.
    pub fn new<'s>(batch_size: usize, subreddits: impl IntoIterator<Item = &'s str>) -> Self {
        let mut map = HashMap::new();

        for sub in subreddits {
            map.insert(sub.to_owned(), SubredditData::default());
        }

        Self {
            batch_size,
            subreddits: map,
            _marker: Default::default(),
        }
    }
}

/// How the subreddit's submissions should be fetched
pub enum FetchMethod {
    /// Fetch each subreddit sequentially, one at a time
    Naive,
    /// Attempt to combine subreddits into multi-reddits (e.g. /r/one+two+three)
    Multi,
}

impl<T> SubmissionStream<T>
where
    T: SubmissionInfo,
{
    /// Fetches the next batch of submissions from this stream
    pub async fn get_next_batch<C: SubmissionsClient<T>>(
        &mut self,
        method: FetchMethod,
        now_utc: f64,
        client: &mut C,
    ) -> Result<Vec<T>, RouxError> {
        match method {
            FetchMethod::Naive => self.naive_next_batch(now_utc, client).await,
            FetchMethod::Multi => self.multi_next_batch(now_utc, client).await,
        }
    }

    fn is_initial_batch(&self) -> bool {
        self.subreddits.values().all(|v| v.seen_queue.len() == 0)
    }

    fn current_batch_size(&self) -> usize {
        if self.is_initial_batch() {
            100
        } else {
            self.batch_size
        }
    }

    /// Fetches the next batch of submissions from this stream.
    ///
    /// This naively fetches each subreddit's submissions one request at a time.
    async fn naive_next_batch<C: SubmissionsClient<T>>(
        &mut self,
        _now_utc: f64,
        client: &mut C,
    ) -> Result<Vec<T>, RouxError> {
        let mut batch = Vec::new();

        let batch_size = self.current_batch_size();

        for (sub, data) in &mut self.subreddits {
            let posts = client.fetch_submissions_for(&sub, batch_size).await?;
            'post: for post in posts {
                for seen in data.seen_queue.iter() {
                    if seen.id == post.id() {
                        continue 'post;
                    }
                }

                let simple = PostInfo {
                    id: post.id().to_owned(),
                    created_utc: post.created_utc(),
                };

                data.seen_queue.push_back(simple);
                batch.push(post);
            }
        }

        batch.sort_unstable_by(|a, b| a.created_utc().total_cmp(&b.created_utc()));

        Ok(batch)
    }

    async fn multi_next_batch<C: SubmissionsClient<T>>(
        &mut self,
        now_utc: f64,
        client: &mut C,
    ) -> Result<Vec<T>, RouxError> {
        let mut batch: Vec<T> = Vec::new();

        // We want to use as few requests as possible to collect all of the subreddits' next data.
        // However, we need to ensure we don't miss posts for subreddits that have high traffic.
        //
        // As such, we estimate the number of new posts to each subreddit, and then start at
        // the subreddits with the lowest traffic, concatening them together into one multi-sub request.

        struct SortableSub<'a> {
            name: &'a str,
            // data: &'a SubredditData,
            estimated_new_posts: usize,
        }

        impl<'a> std::fmt::Debug for SortableSub<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("SortableSub")
                    .field("name", &self.name)
                    .field("estimated_new_posts", &self.estimated_new_posts)
                    .finish()
            }
        }

        impl<'a> SortableSub<'a> {
            fn new(now: f64, (name, data): (&'a String, &'a SubredditData)) -> Self {
                let latest = data
                    .seen_queue
                    .youngest()
                    .map(|v| v.created_utc)
                    .unwrap_or(now);

                let seconds_since = now - latest;

                // println!(
                //     "{name} latest={latest} since = {seconds_since}, per_sec = {}",
                //     data.submissions_per_second()
                // );

                Self {
                    estimated_new_posts: (data.submissions_per_second() * seconds_since) as usize,
                    name,
                    // data,
                }
            }
        }

        let mut subs: Vec<_> = self
            .subreddits
            .iter()
            .map(|entry| SortableSub::new(now_utc, entry))
            .collect();

        subs.sort_unstable_by_key(|a| a.estimated_new_posts);

        let mut next_batch_size = 0;
        let mut next_batch: Vec<SortableSub<'_>> = Vec::new();

        //println!("{subs:#?}");

        macro_rules! fetch_batch {
            () => {
                fetch_batch!(self.current_batch_size())
            };
            ($batch_size:expr) => {
                #[cfg(test)] // make it easier to assert stable request
                next_batch.sort_unstable_by_key(|v| v.name);

                let mut name = String::new();
                for sub in next_batch.drain(..) {
                    if name.len() > 0 {
                        name.push('+');
                    }
                    name.push_str(sub.name);
                }

                let posts = client.fetch_submissions_for(&name, $batch_size).await?;
                batch.extend(posts);
            };
        }

        while let Some(sub) = subs.pop() {
            if sub.estimated_new_posts > self.batch_size {
                // println!(
                //     "[roux: sub-stream] {}: {} > {}",
                //     sub.name, sub.estimated_new_posts, self.batch_size
                // );
                // Too large, just do it stand-alone.
                let posts = client
                    .fetch_submissions_for(sub.name, std::cmp::min(sub.estimated_new_posts, 100))
                    .await?;

                batch.extend(posts);
                continue;
            }

            if (sub.estimated_new_posts + next_batch_size) > self.batch_size {
                // println!(
                //     "[roux: sub-stream] {} + {} > {}",
                //     sub.estimated_new_posts, next_batch_size, self.batch_size
                // );
                fetch_batch!();
                next_batch_size = 0;
            }

            next_batch_size += sub.estimated_new_posts;
            next_batch.push(sub);
        }

        if next_batch.len() > 0 {
            fetch_batch!();
        }

        batch.retain(|post| {
            let Some(data) = self.subreddits.get_mut(post.subreddit()) else {
                return false;
            };

            for seen in data.seen_queue.iter() {
                if seen.id == post.id() {
                    return false;
                }
            }

            let simple = PostInfo {
                id: post.id().to_owned(),
                created_utc: post.created_utc(),
            };

            // println!("new {}: {simple:?}", post.subreddit());

            data.seen_queue.push_back(simple);
            true
        });

        batch.sort_unstable_by(|a, b| a.created_utc().total_cmp(&b.created_utc()));

        Ok(batch)
    }
}

/// Some client that can be used to fetch a subreddit's submissions
pub trait SubmissionsClient<T> {
    /// Fetch the specified number of submissions in the subreddit.
    ///
    /// Note that `subreddits` may contain one or more subreddits, separated by `+`.
    async fn fetch_submissions_for(
        &mut self,
        subreddits: &str,
        num: usize,
    ) -> Result<Vec<T>, RouxError>;
}

macro_rules! impl_client {
    ($($name:ident),* $(,)?) => {
        $(
            impl SubmissionsClient<crate::models::Submission<Self>> for $name {
                async fn fetch_submissions_for(
                    &mut self,
                    subreddits: &str,
                    num: usize,
                ) -> Result<Vec<crate::models::Submission<Self>>, RouxError> {
                    // println!("~~ {subreddits:?} : {num}");

                    let mut listing = self
                        .subreddit(subreddits)
                        .latest(Some(FeedOption::new().limit(num as u32)))
                        .await?;

                    listing.children.reverse();

                    Ok(listing.children)
                }
            }
        )*
    };
}

impl_client!(UnauthedClient, AuthedClient, OAuthClient);

#[derive(Debug)]
struct Queue<T, const MAX: usize>(VecDeque<T>);

impl<T, const MAX: usize> Default for Queue<T, MAX> {
    fn default() -> Self {
        Self(VecDeque::with_capacity(MAX))
    }
}

impl<T, const MAX: usize> Queue<T, MAX> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[cfg(test)]
    pub fn oldest(&self) -> Option<&T> {
        self.0.front()
    }

    pub fn youngest(&self) -> Option<&T> {
        self.0.back()
    }

    pub fn push_back(&mut self, value: T) -> Option<T> {
        if self.0.len() >= MAX {
            let front = self.0.pop_front();
            self.0.push_back(value);
            front
        } else {
            self.0.push_back(value);
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

/// Describes information a submission is expected to have.
pub trait SubmissionInfo {
    /// The unique identifier of the submission.
    fn id(&self) -> &str;
    /// The name of the subreddit the post was made in
    fn subreddit(&self) -> &str;
    /// The UTC timestamp of when the post was created
    fn created_utc(&self) -> f64;
}

impl<T> SubmissionInfo for crate::models::Submission<T> {
    fn id(&self) -> &str {
        self.id().as_str()
    }

    fn subreddit(&self) -> &str {
        self.subreddit().as_str()
    }

    fn created_utc(&self) -> f64 {
        self.created_utc()
    }
}

#[derive(Debug)]
struct PostInfo {
    /// A unique identifier for the submission
    id: String,
    /// When the submission was created
    created_utc: f64,
}

#[derive(Default)]
struct SubredditData {
    seen_queue: Queue<PostInfo, 100>,
}

impl std::fmt::Debug for SubredditData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubredditData")
            .field("per_sec", &self.submissions_per_second())
            .field("seen_queue", &self.seen_queue)
            .finish()
    }
}

impl SubredditData {
    fn submissions_per_second(&self) -> f64 {
        if self.seen_queue.len() == 0 {
            return 0.0;
        }

        let avg_time_between_posts = self
            .seen_queue
            .iter()
            .map_windows::<_, f64, 2>(|&[a, b]| b.created_utc - a.created_utc)
            .sum::<f64>()
            / (self.seen_queue.len() - 1) as f64;

        1.0 / avg_time_between_posts
    }
}

/// Returns the number of seconds since the UNIX epoch.
pub fn now_utc() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::client::UnauthedClient;

    use super::*;

    #[test]
    fn test_queue() {
        let mut queue: Queue<i32, 10> = Queue::default();

        // dropped out of queue
        queue.push_back(0);
        queue.push_back(1);
        // final ten:
        queue.push_back(2);
        queue.push_back(3);
        queue.push_back(4);
        queue.push_back(5);
        queue.push_back(6);
        queue.push_back(7);
        queue.push_back(8);
        queue.push_back(9);
        queue.push_back(10);
        queue.push_back(11);

        assert_eq!(queue.youngest(), Some(&11));
        assert_eq!(queue.oldest(), Some(&2));

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);
    }

    #[derive(Debug, Clone, PartialEq)]
    struct DebugSubmission {
        subreddit: String,
        id: String,
        created_utc: f64,
    }

    impl DebugSubmission {
        fn example_post_infos(
            start_from: usize,
            created_offset: f64,
        ) -> impl Iterator<Item = PostInfo> {
            let mut idx = start_from;
            std::iter::from_fn(move || {
                let id = format!("{idx:0000}");

                let post = PostInfo {
                    id,
                    created_utc: created_offset + (idx - start_from) as f64 * 5.0,
                };

                idx += 1;

                Some(post)
            })
        }

        fn examples(
            start_from: usize,
            created_offset: f64,
            subreddit: &'static str,
        ) -> impl Iterator<Item = Self> {
            Self::example_post_infos(start_from, created_offset).map(|v| Self {
                subreddit: subreddit.to_owned(),
                id: v.id,
                created_utc: v.created_utc,
            })
        }
    }

    impl SubmissionInfo for DebugSubmission {
        fn id(&self) -> &str {
            &self.id
        }

        fn subreddit(&self) -> &str {
            &self.subreddit
        }

        fn created_utc(&self) -> f64 {
            self.created_utc
        }
    }

    #[derive(Default)]
    struct DebugClient {
        visible: VecDeque<DebugSubmission>,
        fetches: Vec<String>,
    }

    impl DebugClient {
        fn push(&mut self, iter: impl IntoIterator<Item = DebugSubmission>) {
            for item in iter {
                self.visible.push_back(item);
            }
        }
    }

    impl SubmissionsClient<DebugSubmission> for DebugClient {
        async fn fetch_submissions_for(
            &mut self,
            subreddits: &str,
            num: usize,
        ) -> Result<Vec<DebugSubmission>, RouxError> {
            println!("~~ {subreddits:?} : {num}");
            self.fetches.push(subreddits.to_owned());
            let mut v = Vec::with_capacity(num);

            for item in self.visible.iter().rev() {
                if subreddits.contains(item.subreddit.as_str()) {
                    v.push(item.clone());
                }

                if v.len() >= num {
                    break;
                }
            }

            v.reverse();

            Ok(v)
        }
    }

    #[test]
    fn test_subreddit_data_per_sec() {
        let mut data = SubredditData::default();

        for simple in DebugSubmission::example_post_infos(0, 0.0).take(5) {
            data.seen_queue.push_back(simple);
        }

        assert_eq!(data.submissions_per_second(), 0.2);
    }

    macro_rules! assert_has_posts {
        ($vec:ident, [$(
            $id:literal, $created:literal
        );* $(;)?] $(, $rest:tt)?) => {
            assert_eq!($vec, vec![
                $(
                   DebugSubmission {
                       id: String::from($id),
                       subreddit: String::from("sub1"),
                       created_utc: $created,
                   }
                ),*
            ] $(, $rest)?)
        };
        ($vec:ident, [$(
            $id:literal, $created:literal, $sub:literal
        );* $(;)?] $(, $rest:tt)?) => {
            assert_eq!($vec, vec![
                $(
                   DebugSubmission {
                       id: String::from($id),
                       subreddit: String::from($sub),
                       created_utc: $created,
                   }
                ),*
            ] $(, $rest)?)
        };
    }

    #[tokio::test]
    async fn test_single_subreddit_fetch() {
        let mut stream = SubmissionStream::<DebugSubmission>::new(5, std::iter::once("sub1"));

        let iter = &mut DebugSubmission::examples(0, 0.0, "sub1");
        let mut client = DebugClient::default();
        client.push(iter.take(5));

        let posts = stream
            .get_next_batch(FetchMethod::Naive, 20.0, &mut client)
            .await
            .unwrap();

        assert_has_posts!(posts, [
            "0", 0.0;
            "1", 5.0;
            "2", 10.0;
            "3", 15.0;
            "4", 20.0;
        ]);

        let posts = stream
            .get_next_batch(FetchMethod::Naive, 25.0, &mut client)
            .await
            .unwrap();

        assert_has_posts!(posts, [], "no new posts - should be empty");

        client.push(iter.take(2));
        let posts = stream
            .get_next_batch(FetchMethod::Naive, 30.0, &mut client)
            .await
            .unwrap();

        assert_has_posts!(posts, [
            "5", 25.0;
            "6", 30.0;
        ], "only two new posts - should emit them");

        let posts = stream
            .get_next_batch(FetchMethod::Naive, 40.0, &mut client)
            .await
            .unwrap();
        assert_has_posts!(posts, [], "still new posts - should be empty");
    }

    #[tokio::test]
    async fn test_double_subreddit_fetch_naive() {
        let mut stream = SubmissionStream::<DebugSubmission>::new(5, vec!["sub1", "sub2"]);

        let sub1 = &mut DebugSubmission::examples(0, 0.0, "sub1");
        let sub2 = &mut DebugSubmission::examples(50, 2.5, "sub2");

        let mut client = DebugClient::default();

        client.push(sub1.next());
        client.push(sub2.next());
        client.push(sub1.next());
        client.push(sub2.next());
        client.push(sub1.next());

        let posts = stream
            .get_next_batch(FetchMethod::Naive, 20.0, &mut client)
            .await
            .unwrap();

        assert_eq!(client.fetches.len(), 2, "one fetch per sub");

        assert_has_posts!(posts, [
            "0", 0.0, "sub1";
            "50", 2.5, "sub2";
            "1", 5.0, "sub1";
            "51", 7.5, "sub2";
            "2", 10.0, "sub1";
        ]);

        let posts = stream
            .get_next_batch(FetchMethod::Naive, 25.0, &mut client)
            .await
            .unwrap();

        assert_eq!(client.fetches.len(), 4, "one fetch per sub");
        assert_has_posts!(posts, [], "no new posts - should be empty");

        client.push(sub2.next());
        client.push(sub1.next());

        let posts = stream
            .get_next_batch(FetchMethod::Naive, 30.0, &mut client)
            .await
            .unwrap();

        assert_eq!(client.fetches.len(), 6, "one fetch per sub");
        assert_has_posts!( posts, [
            "52", 12.5, "sub2";
            "3", 15.0, "sub1";
        ], "only two new posts - should emit them");

        let posts = stream
            .get_next_batch(FetchMethod::Naive, 40.0, &mut client)
            .await
            .unwrap();

        assert_eq!(client.fetches.len(), 8, "one fetch per sub");
        assert_has_posts!(posts, [], "still new posts - should be empty");
    }

    #[tokio::test]
    async fn test_double_subreddit_fetch_multi() {
        let mut stream = SubmissionStream::<DebugSubmission>::new(10, vec!["sub1", "sub2"]);

        let sub1 = &mut DebugSubmission::examples(0, 0.0, "sub1");
        let sub2 = &mut DebugSubmission::examples(50, 2.5, "sub2");

        let mut client = DebugClient::default();

        client.push(sub1.next());
        client.push(sub2.next());
        client.push(sub1.next());
        client.push(sub2.next());
        client.push(sub1.next());

        let posts = stream
            .get_next_batch(FetchMethod::Multi, 15.0, &mut client)
            .await
            .unwrap();

        assert_eq!(client.fetches, vec!["sub1+sub2"]);

        assert_has_posts!(posts, [
            "0", 0.0, "sub1";
            "50", 2.5, "sub2";
            "1", 5.0, "sub1";
            "51", 7.5, "sub2";
            "2", 10.0, "sub1";
        ]);

        let posts = stream
            .get_next_batch(FetchMethod::Multi, 20.0, &mut client)
            .await
            .unwrap();

        assert_eq!(client.fetches, vec!["sub1+sub2", "sub1+sub2"]);
        assert_has_posts!(posts, [], "no new posts - should be empty");

        client.push(sub2.next());
        client.push(sub1.next());
        client.push(sub2.next());
        client.push(sub1.next());
        client.push(sub2.next());

        let posts = stream
            .get_next_batch(FetchMethod::Multi, 40.0, &mut client)
            .await
            .unwrap();

        // Enough time has passed since the last seen posts that
        // the estimate for both subs combined is larger than the batch size.
        // As such, the client should fall back to single requests
        assert_eq!(
            client.fetches,
            vec!["sub1+sub2", "sub1+sub2", "sub1", "sub2"]
        );

        assert_has_posts!( posts, [
            "52", 12.5, "sub2";
            "3", 15.0, "sub1";
            "53", 17.5, "sub2";
            "4", 20.0, "sub1";
            "54", 22.5, "sub2";
        ], "new posts should be emit");

        let posts = stream
            .get_next_batch(FetchMethod::Multi, 45.0, &mut client)
            .await
            .unwrap();

        assert_eq!(
            client.fetches,
            vec![
                // initial request, no prior knowledge
                "sub1+sub2",
                // only short duration to first request,
                // combined number of posts estimated
                // to be below batch size
                "sub1+sub2",
                // longer duration, combined estimate is too big for
                // one request. split it into two
                "sub1",
                "sub2",
                // back to shorter duration to latest post(s),
                // so one request should do
                "sub1+sub2"
            ]
        );

        assert_has_posts!(posts, [], "still new posts - should be empty");
    }

    #[tokio::test]
    async fn test_with_client() -> Result<(), RouxError> {
        let mut client = crate::client::UnauthedClient::new()?;

        let mut stream = SubmissionStream::<crate::models::Submission<UnauthedClient>>::new(
            25,
            ["discordapp", "rust", "pics"],
        );

        for _ in 0..10 {
            let now = super::now_utc();
            let posts = stream
                .get_next_batch(FetchMethod::Multi, now, &mut client)
                .await?;

            println!("now: {now}");

            for post in &posts {
                println!(
                    "{} @ {}: {}",
                    post.subreddit(),
                    post.created_utc(),
                    post.title()
                );
            }

            println!("\n----\n");

            for (sub, data) in &stream.subreddits {
                println!("{sub}: {data:?}");
            }

            tokio::time::sleep(Duration::from_secs(10)).await;
        }

        Ok(())
    }
}
