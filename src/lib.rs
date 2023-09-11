struct NTSubscriptionOptions {
    periodic_rate_seconds: f32,
    all: bool,
    topics_only: bool,
    prefix: bool,
}

impl NTSubscriptionOptions {}

struct NTSubscription {
    topic: String,
    options: NTSubscriptionOptions,
    uid: f32,
}

impl NTSubscription {
    pub fn get_topic(&self) -> &String {
        &self.topic
    }

    pub fn get_options(&self) -> &NTSubscriptionOptions {
        &self.options
    }

    pub fn get_uid(&self) -> f32 {
        self.uid
    }

    pub fn from(topic: String, options: NTSubscriptionOptions, uid: f32) -> Self {
        NTSubscription {
            topic,
            options,
            uid,
        }
    }
}

pub trait NT4 {}

