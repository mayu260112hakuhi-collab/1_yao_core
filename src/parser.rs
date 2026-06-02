use regex::Regex;
use std::fs;
use crate::math;

pub struct YaoyorozuEngine {
    pub is_streaming: bool,
    pub is_launcher: bool,
}

impl YaoyorozuEngine {
    pub fn new() -> Self {
        Self { is_streaming: false, is_launcher: true }
    }

    pub fn parse(&self, content: &str) -> String {
        self.process_all_includes(content)
    }

    fn process_all_includes(&self, content: &str) -> String {
        // インクルード処理の基礎実装
        content.to_string()
    }

    fn condition_is_true(&self, condition: &str) -> bool {
        let condition = condition.trim();
        if condition == "ここがランチャーなら" { return self.is_launcher; }

        if let Some(value) = math::parse_wasan_expression(condition) {
            return value != 0.0;
        }
        false
    }
    
    fn parse_number(&self, value: &str) -> Option<u32> {
        value.parse::<u32>().ok()
    }
}