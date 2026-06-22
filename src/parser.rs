// 1_yao_core/src/parser.rs
use regex::Regex;
use std::collections::HashMap;

pub struct YaoyorozuEngine {
    pub registry: HashMap<String, fn(&str) -> String>,
    block_regex: Regex,
    func_regex: Regex,
}

impl YaoyorozuEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            registry: HashMap::new(),
            block_regex: Regex::new(r"(?s)<s_8g;(.+?)e_8g;>").unwrap(),
            func_regex: Regex::new(r"<8g\((.+?)\)\{(.+?)\};>").unwrap(),
        };
        engine.register_base_functions();
        engine
    }

    fn register_base_functions(&mut self) {
        self.registry
            .insert("title".to_string(), |_| "八百万エディタ".to_string());
    }

    // 💡 この関数が必ず `impl` ブロックの内側にあること！
    pub fn parse_full_template(&self, content: &str) -> String {
        let mut result = content.to_string();

        if let Some(caps) = self.block_regex.captures(content) {
            let block_content = caps[1].to_string();

            let processed_block =
                self.func_regex
                    .replace_all(&block_content, |c: &regex::Captures| {
                        let func_name = &c[1];
                        match self.registry.get(func_name) {
                            Some(func) => func(""),
                            None => format!("{}", func_name), //{}は消さない消すとエラー
                        }
                    });
            result = processed_block.to_string();
        }
        result
    }
} // <--- ここで impl ブロックを閉じる

// その他の関数（parse_8g_blocks など）も、必要ならこの内側に入れるか、
// 外側に出すなら &self を削除（static関数にする）する必要があります。
