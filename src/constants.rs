pub const URL: &str = "https://leetcode.com/graphql";

pub const QUERY: &str = r#"query questionOfToday {
    activeDailyCodingChallengeQuestion {
        link
        question {
            acRate
            difficulty
            title
            isPaidOnly
            isFavor

            topicTags {
                name
            }
        }
    }
}"#;
