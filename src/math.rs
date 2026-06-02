use regex::Regex;

// 外部から呼べるように pub にします
pub fn parse_wasan_expression(expression: &str) -> Option<f64> {
    let normalized = expression
        .chars()
        .filter_map(|ch| match ch {
            '０'..='９' => Some(((ch as u32 - '０' as u32) + b'0' as u32) as u8 as char),
            '＋' => Some('+'),
            '－' => Some('-'),
            '＊' => Some('*'),
            '／' => Some('/'),
            '（' => Some('('),
            '）' => Some(')'),
            c if c.is_ascii_whitespace() => None,
            c => Some(c),
        })
        .collect::<String>();

    let expression = normalized
        .replace("足す", "+")
        .replace("引く", "-")
        .replace("掛ける", "*")
        .replace("かける", "*")
        .replace("割る", "/");

    if !Regex::new(r#"^[0-9.+\-*/()]+$"#).unwrap().is_match(&expression) {
        return None;
    }

    evaluate_expression(&expression)
}

pub fn evaluate_expression(expr: &str) -> Option<f64> {
    #[derive(Debug, Clone, Copy)]
    enum Op { Add, Sub, Mul, Div }
    #[derive(Debug, Clone)]
    enum Token { Number(f64), Op(Op), LParen, RParen }

    fn tokenize(expr: &str) -> Option<Vec<Token>> { /* ...元のtokenize実装をここにコピペ... */ }
    fn parse_expression(tokens: &[Token], pos: &mut usize) -> Option<f64> { /* ... */ }
    fn parse_term(tokens: &[Token], pos: &mut usize) -> Option<f64> { /* ... */ }
    fn parse_factor(tokens: &[Token], pos: &mut usize) -> Option<f64> { /* ... */ }

    // ...以下、元の evaluate_expression の残りの処理...
    let tokens = tokenize(expr)?;
    let mut pos = 0;
    let value = parse_expression(&tokens, &mut pos)?;
    if pos == tokens.len() { Some(value) } else { None }
}