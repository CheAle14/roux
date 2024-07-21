use std::{
    fmt::Debug,
    str::FromStr,
    time::{Duration, Instant},
};

use super::req::sleep;
use reqwest::header::HeaderMap;

pub struct Ratelimit {
    remaining: f64,
    used: u64,
    next_request: Instant,
    next_reset: Instant,
}

impl Ratelimit {
    const WINDOW: f64 = 600.0;

    pub fn new() -> Self {
        Self {
            remaining: 100.0,
            used: 0,
            next_request: Instant::now(),
            next_reset: Instant::now() + Duration::from_secs(Self::WINDOW as u64),
        }
    }

    #[maybe_async::maybe_async]
    pub async fn delay(&self) {
        let now = Instant::now();
        let Some(diff) = self.next_request.checked_duration_since(now.clone()) else {
            return;
        };

        println!("[RL] Sleeping for {diff:?}");
        sleep(diff).await;
    }

    fn get<T>(headers: &HeaderMap, name: &str) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let text = headers.get(name).unwrap().to_str().unwrap();

        text.parse().unwrap()
    }

    pub fn update(&mut self, headers: &HeaderMap) {
        if !headers.contains_key("X-Ratelimit-Remaining") {
            self.remaining -= 1.0;
            self.used += 1;
            return;
        };

        let now = Instant::now();

        let reset_seconds = Self::get(headers, "X-Ratelimit-Reset");
        self.remaining = Self::get(headers, "X-Ratelimit-Remaining");
        self.used = Self::get(headers, "X-Ratelimit-Used");

        self.next_reset = now + Duration::from_secs(reset_seconds);

        if self.remaining <= 0.0 {
            self.next_request = self.next_reset.clone();
            return;
        }

        let remain = self.remaining as f64;
        let used = self.used as f64;

        // The total number of queries that we can make within the window time
        let allowed = remain + used;

        // The average number of seconds between each request
        let average_seconds_per_request = Self::WINDOW / allowed;

        // How many seconds of the window we have already used
        let seconds_taken_so_far = average_seconds_per_request * used;

        // How much of the window does this leave us?
        let window_remain = Self::WINDOW - seconds_taken_so_far;

        // If less of the window actually remains than we think it does,
        // then this value will be positive and thus we need to sleep for that duration.
        let seconds_delay = reset_seconds as f64 - window_remain;

        let seconds_delay = clamp(0.0, seconds_delay, 10.0);

        let ms_delay = seconds_delay * 1000.0;
        let us_delay = ms_delay * 1000.0;

        let next_request = now + Duration::from_micros(us_delay as u64);

        // but don't wait past when the window actually resets.
        self.next_request = std::cmp::min(next_request, self.next_reset);
    }
}

fn clamp(min: f64, value: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
