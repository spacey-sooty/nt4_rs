pub mod client {
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NT4SubscriptionOptions {
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

        pub fn get_periodic_rate_seconds(&self) -> f32 {
            self.periodic_rate_seconds
        }

        pub fn get_all(&self) -> bool {
            self.all
        }

        pub fn get_topics_only(&self) -> bool {
            self.topics_only
        }

        pub fn get_prefix(&self) -> bool {
            self.prefix
        }

        pub fn to_json(&self) -> serde_json::Value {
            serde_json::to_value(self).unwrap()
        }

        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NT4Subscription {
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

        pub fn from_json(json: &str) -> Self {
            serde_json::from_str(json).unwrap()
        }

        pub fn to_json(&self) -> serde_json::Value {
            serde_json::to_value(self).unwrap()
        }
    }

    pub trait NT4Client {}
}
