// 1_yao_core/src/parser.rs
use regex::Regex;
use serde::Serialize;
use crate::math;

pub struct YaoyorozuEngine {
    pub is_streaming: bool,
    pub is_launcher: bool,
}

/// コア側で定義する、解析済みのUI要素のデータ（これをゲーム側に渡す）
#[derive(Debug, Clone, Serialize)]
pub struct RawUiNode {
    pub tag: String,       // "区画", "ボタン" など
    pub name: String,      // 「メインメニュー」 などの名前
    pub properties: Vec<(String, String)>, // ("横幅", "300") などのキーと値のペア
    pub children: Vec<RawUiNode>,          // 子要素のリスト
}

impl YaoyorozuEngine {
    pub fn new() -> Self {
        Self { is_streaming: false, is_launcher: true }
    }

    pub fn parse(&self, content: &str) -> String {
        self.process_all_includes(content)
    }

    /// 💡 新機能：.8gのテキストからUIのツリー構造（RawUiNode）を解析する関数
    pub fn parse_ui_script(&self, content: &str) -> Vec<RawUiNode> {
        let mut nodes = Vec::new();
        
        // 簡易的な行単位の解析ルール（正規表現）
        // 例: 区画「メインメニュー」 {
        let block_regex = Regex::new(r"([^\x00-\x7F]+)「([^」]+)」\s*\{").unwrap();
        // 例: 横幅: 300,
        let prop_regex = Regex::new(r"([^\x00-\x7F]+):\s*([^,}\n]+)").unwrap();

        let lines: Vec<&str> = content.lines().map(|l| l.trim()).collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            
            if line.contains("//") || line.is_empty() {
                i += 1;
                // 挑戦コード用文章はコメント用に先頭に // を付けるルールに配慮
                continue;
            }

            if let Some(caps) = block_regex.captures(line) {
                let tag = caps[1].to_string();
                let name = caps[2].to_string();
                let mut properties = Vec::new();
                
                // 次の行から中括弧が閉じるまでプロパティを読み込む
                i += 1;
                while i < lines.len() && !lines[i].contains("}") {
                    let prop_line = lines[i];
                    if let Some(p_caps) = prop_regex.captures(prop_line) {
                        properties.push((p_caps[1].trim().to_string(), p_caps[2].trim().to_string()));
                    }
                    i += 1;
                }

                nodes.push(RawUiNode {
                    tag,
                    name,
                    properties,
                    children: Vec::new(), // 現状は1階層のみ。今後再帰化可能
                });
            }
            i += 1;
        }

        nodes
    }

    fn process_all_includes(&self, content: &str) -> String {
        content.to_string()
    }

    #[allow(dead_code)]
    fn condition_is_true(&self, condition: &str) -> bool {
        let condition = condition.trim();
        if condition == "ここがランチャーなら" { return self.is_launcher; }

        if let Some(value) = math::parse_wasan_expression(condition) {
            return value != 0.0;
        }
        false
    }
    
    #[allow(dead_code)]
    fn parse_number(&self, value: &str) -> Option<u32> {
        value.parse::<u32>().ok()
    }
}