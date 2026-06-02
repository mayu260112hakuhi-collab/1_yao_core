use regex::Regex;
use std::fs;
use std::path::Path;
use crate::math; // mathモジュールを呼び出す

pub struct YaoyorozuEngine {
    pub is_streaming: bool,
    pub is_launcher: bool,
}

impl YaoyorozuEngine {
    pub fn new() -> Self {
        Self { is_streaming: false, is_launcher: true }
    }

    pub fn parse(&self, content: &str) -> String {
        let mut result = self.process_all_includes(content);
        // ... (以下、元の parse メソッドの内容をそのまま移植)
        result
    }

    // ... (process_all_includes, process_loops, process_conditionals をここに移植)

    fn condition_is_true(&self, condition: &str) -> bool {
        let condition = condition.trim();
        if condition == "ここがランチャーなら" { return self.is_launcher; }

        // ここでmathモジュールを活用！
        if let Some(value) = math::parse_wasan_expression(condition) {
            return value != 0.0;
        }
        // ...
    }
    
    fn parse_number(&self, value: &str) -> Option<u32> { /* ...そのまま移植... */ }
}