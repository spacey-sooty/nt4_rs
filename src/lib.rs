pub mod client {
    struct NT4SubscriptionOptions {
        periodic_rate_seconds: f32,
        all: bool,
        topics_only: bool,
        prefix: bool,
    }

    impl NT4SubscriptionOptions {
        pub fn from(
            periodic_rate_seconds: f32,
            all: bool,
            topics_only: bool,
            prefix: bool,
        ) -> Self {
            NT4SubscriptionOptions {
                periodic_rate_seconds,
                all,
                topics_only,
                prefix,
            }
        }
    }

    struct NT4Subscription {
        topic: String,
        options: NT4SubscriptionOptions,
        uid: f32,
    }

    impl NT4Subscription {
        pub fn get_topic(&self) -> &String {
            &self.topic
        }

        pub fn get_options(&self) -> &NT4SubscriptionOptions {
            &self.options
        }

        pub fn get_uid(&self) -> f32 {
            self.uid
        }

        pub fn from(topic: String, options: NT4SubscriptionOptions, uid: f32) -> Self {
            NT4Subscription {
                topic,
                options,
                uid,
            }
        }
    }

    pub trait NT4Client {}
}
