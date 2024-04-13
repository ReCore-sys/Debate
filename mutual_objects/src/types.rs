
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct User {
        pub username: String,
        pub password: String,
        pub uuid: String,
        pub pfp: String,
        pub email: String,
        pub bio: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Message {
        pub author: String,
        pub content: String,
        pub timestamp: String,
        pub channel: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct LoginRequest {
        pub username: String,
        pub password: String,
    }
