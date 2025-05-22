use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Term {
    pub term_id: i64,        // 术语id
    pub original: String,    // 原文
    pub translation: String, // 译文
    pub desc: String,        // 对术语的描述
}

#[derive(Debug, Deserialize, Serialize)]
struct TermCreatePayload {
    pub original: String,    // 原文
    pub translation: String, // 译文
    pub desc: String,        // 对术语的描述
}

#[derive(Debug, Deserialize, Serialize)]
struct TermUpdatePayload {
    pub term_id: i64,                // 术语id
    pub original: Option<String>,    // 原文
    pub translation: Option<String>, // 译文
    pub desc: Option<String>,        // 对术语的描述
}
