use caie_code_rs::{Lexer, ParseConfig, TokenKind};

/// 辅助：把 token 流简化成 "种类描述" 字符串列表，方便断言
fn kind_str(k: &TokenKind) -> String {
    match k {
        TokenKind::Int(i) => format!("Int({})", i),
        TokenKind::Float(f) => format!("Float({})", f),
        TokenKind::Str(s) => format!("Str({:?})", s),
        TokenKind::Char(c) => format!("Char({:?})", c),
        TokenKind::Identifier(s) => format!("Ident({})", s),
        TokenKind::Keyword(k) => format!("Kw({:?})", k),
        TokenKind::Assign => "Assign".into(),
        TokenKind::Operator(o) => format!("Op({:?})", o),
        TokenKind::LParen => "LParen".into(),
        TokenKind::RParen => "RParen".into(),
        TokenKind::LBracket => "LBracket".into(),
        TokenKind::RBracket => "RBracket".into(),
        TokenKind::LBrace => "LBrace".into(),
        TokenKind::RBrace => "RBrace".into(),
        TokenKind::Comma => "Comma".into(),
        TokenKind::Semicolon => "Semicolon".into(),
        TokenKind::Colon => "Colon".into(),
        TokenKind::EOL => "EOL".into(),
        TokenKind::EOF => "EOF".into(),
    }
}

fn lex_kinds(src: &str, config: &ParseConfig) -> Vec<String> {
    let tokens = Lexer::new(src, config).tokenize().expect("lex should succeed");
    tokens.iter().map(|t| kind_str(&t.kind)).collect()
}

// ============ 基本 token ============

#[test]
fn lex_declare_int() {
    // 注意：INTEGER 在 lexer 里是标识符（不是关键字），由 parser 识别为类型
    let kinds = lex_kinds("DECLARE X : INTEGER", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "Kw(Declare)".to_string(),
        "Ident(X)".to_string(),
        "Colon".to_string(),
        "Ident(INTEGER)".to_string(),
        "EOF".to_string(),
    ]);
}

#[test]
fn lex_arrow_assign() {
    // ← 和 <- 都应该是 Assign
    let kinds = lex_kinds("X ← 5\nY <- 10", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "Ident(X)", "Assign", "Int(5)", "EOL",
        "Ident(Y)", "Assign", "Int(10)", "EOF",
    ]);
}

#[test]
fn lex_equal_assign_normal_mode() {
    // 普通模式：= 是赋值，== 是相等
    let config = ParseConfig::normal();
    let kinds = lex_kinds("X = 5", &config);
    assert!(kinds.iter().any(|k| k == "Assign"));
}

#[test]
fn lex_strict_mode_only_arrow() {
    // 严格模式：只允许 ← / <-，不允许 =
    let config = ParseConfig::strict();
    // strict() 默认 allow_equal_assign = false
    let kinds = lex_kinds("X ← 5", &config);
    assert!(kinds.iter().any(|k| k == "Assign"));
    // = 在严格模式下应是相等运算符
    let kinds = lex_kinds("X = 5", &config);
    assert!(kinds.iter().any(|k| k == "Op(Eq)"));
}

// ============ 关键字大小写（关键字永远不敏感）============

#[test]
fn lex_keyword_case_insensitive() {
    // 关键字不论大小写都识别
    let kinds = lex_kinds("if If IF then THEN endif ENDIF", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "Kw(If)", "Kw(If)", "Kw(If)",
        "Kw(Then)", "Kw(Then)",
        "Kw(EndIf)", "Kw(EndIf)",
        "EOF",
    ]);
}

// ============ 标识符大小写配置 ============

#[test]
fn lex_identifier_case_sensitive() {
    // 普通模式：标识符大小写敏感，原样保留
    let kinds = lex_kinds("Count count COUNT", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "Ident(Count)", "Ident(count)", "Ident(COUNT)", "EOF",
    ]);
}

#[test]
fn lex_identifier_case_insensitive() {
    // 严格模式（case_sensitive=false）：标识符转小写规范化
    let config = ParseConfig::strict();
    let kinds = lex_kinds("Count count COUNT", &config);
    assert_eq!(kinds, vec![
        "Ident(count)", "Ident(count)", "Ident(count)", "EOF",
    ]);
}

// ============ 数字 ============

#[test]
fn lex_integer() {
    let kinds = lex_kinds("42 0 12345", &ParseConfig::normal());
    assert_eq!(kinds, vec!["Int(42)", "Int(0)", "Int(12345)", "EOF"]);
}

#[test]
fn lex_float() {
    let kinds = lex_kinds("3.14 0.5 100.0", &ParseConfig::normal());
    assert_eq!(kinds, vec!["Float(3.14)", "Float(0.5)", "Float(100)", "EOF"]);
}

#[test]
fn lex_negative_not_in_lexer() {
    // 负号是 lexer 的 Minus 运算符 + 正数，负数由 parser 处理（Unary）
    let kinds = lex_kinds("-5", &ParseConfig::normal());
    assert_eq!(kinds, vec!["Op(Sub)", "Int(5)", "EOF"]);
}

// ============ 字符串与转义 ============

#[test]
fn lex_string_basic() {
    let kinds = lex_kinds("\"hello world\"", &ParseConfig::normal());
    assert_eq!(kinds, vec!["Str(\"hello world\")", "EOF"]);
}

#[test]
fn lex_string_escape() {
    // \n 应解释成换行符，\t 是制表符
    let config = ParseConfig::normal();
    let tokens = Lexer::new("\"a\\nb\\tc\"", &config).tokenize().unwrap();
    match &tokens[0].kind {
        TokenKind::Str(s) => {
            // 内容应该是: a + 换行 + b + tab + c
            assert_eq!(s.chars().count(), 5);
            assert_eq!(s.chars().nth(0), Some('a'));
            assert_eq!(s.chars().nth(1), Some('\n'));
            assert_eq!(s.chars().nth(2), Some('b'));
            assert_eq!(s.chars().nth(3), Some('\t'));
            assert_eq!(s.chars().nth(4), Some('c'));
        }
        other => panic!("expected Str, got {:?}", other),
    }
}

#[test]
fn lex_string_escape_quote() {
    // \" 应解释成普通引号字符，不结束字符串
    let config = ParseConfig::normal();
    let tokens = Lexer::new("\"say \\\"hi\\\"\"", &config).tokenize().unwrap();
    match &tokens[0].kind {
        TokenKind::Str(s) => assert_eq!(s, "say \"hi\""),
        other => panic!("expected Str, got {:?}", other),
    }
}

#[test]
fn lex_string_unterminated() {
    let config = ParseConfig::normal();
    let result = Lexer::new("\"unterminated", &config).tokenize();
    assert!(result.is_err(), "unterminated string should error");
}

#[test]
fn lex_string_newline_unterminated() {
    // 默认不允许字符串跨行
    let config = ParseConfig::normal();
    let result = Lexer::new("\"line1\nline2\"", &config).tokenize();
    assert!(result.is_err(), "string with raw newline should error");
}

// ============ 字符字面量 ============

#[test]
fn lex_char_basic() {
    let kinds = lex_kinds("'a'", &ParseConfig::normal());
    assert_eq!(kinds, vec!["Char('a')", "EOF"]);
}

#[test]
fn lex_char_escape() {
    // '\n' 应是换行符
    let config = ParseConfig::normal();
    let tokens = Lexer::new("'\\n'", &config).tokenize().unwrap();
    match &tokens[0].kind {
        TokenKind::Char(c) => assert_eq!(*c, '\n'),
        other => panic!("expected Char, got {:?}", other),
    }
}

#[test]
fn lex_char_escape_quote() {
    // '\'' 应是单引号字符
    let config = ParseConfig::normal();
    let tokens = Lexer::new("'\\''", &config).tokenize().unwrap();
    match &tokens[0].kind {
        TokenKind::Char(c) => assert_eq!(*c, '\''),
        other => panic!("expected Char, got {:?}", other),
    }
}

// ============ 运算符（多字符贪心匹配）============

#[test]
fn lex_multichar_operators() {
    // <= <> >= <- 这些应该整体识别，而不是拆成 < = 等
    let kinds = lex_kinds("<= <> >= <- < >", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "Op(Lte)", "Op(Neq)", "Op(Gte)", "Assign",
        "Op(Lt)", "Op(Gt)", "EOF",
    ]);
}

#[test]
fn lex_word_operators() {
    // CAIE 字母风格的运算符。注意：
    //   DIV = 整数除法（对应 Op::FullDiv，与 / 的 Op::Div 区分）
    //   /   = 除法（Op::Div）
    //   MOD AND OR NOT = 取模 / 逻辑运算
    // lexer 只验证 token 识别，运算语义在 binary_op 实现。
    let kinds = lex_kinds("A DIV B  C MOD D  X AND Y  P OR Q  NOT Z",
                          &ParseConfig::normal());
    assert!(kinds.contains(&"Op(FullDiv)".to_string()), "should recognize DIV (integer division)");
    assert!(kinds.contains(&"Op(Mod)".to_string()), "should recognize MOD");
    assert!(kinds.contains(&"Op(And)".to_string()), "should recognize AND");
    assert!(kinds.contains(&"Op(Or)".to_string()), "should recognize OR");
    assert!(kinds.contains(&"Op(Not)".to_string()), "should recognize NOT");
}

#[test]
fn lex_concat_operator() {
    // & 拼接符
    let kinds = lex_kinds("\"a\" & \"b\"", &ParseConfig::normal());
    assert_eq!(kinds, vec!["Str(\"a\")", "Op(Concat)", "Str(\"b\")", "EOF"]);
}

// ============ 标点与括号 ============

#[test]
fn lex_punctuation() {
    let kinds = lex_kinds("( ) [ ] { } , ; :", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "LParen", "RParen", "LBracket", "RBracket",
        "LBrace", "RBrace", "Comma", "Semicolon", "Colon", "EOF",
    ]);
}

// ============ 注释 ============

#[test]
fn lex_comment() {
    // // 注释到行尾，不产生 token
    let kinds = lex_kinds("X ← 5 // 这是注释\nY ← 10", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "Ident(X)", "Assign", "Int(5)", "EOL",
        "Ident(Y)", "Assign", "Int(10)", "EOF",
    ]);
}

#[test]
fn lex_comment_only_line() {
    // 整行注释应该只产生 EOL（或啥也不产生，取决于实现）
    let kinds = lex_kinds("// just a comment\nX ← 1", &ParseConfig::normal());
    // 注释行后是换行，然后 X ← 1
    assert!(kinds.iter().any(|k| k == "Ident(X)"));
    assert!(kinds.iter().any(|k| k == "Int(1)"));
}

// ============ 换行 ============

#[test]
fn lex_newlines() {
    let kinds = lex_kinds("X ← 1\n\n\nY ← 2", &ParseConfig::normal());
    // 连续空行各产生一个 EOL（lexer 不折叠，由 parser 处理）
    assert!(kinds.iter().filter(|k| k.as_str() == "EOL").count() >= 1);
}

// ============ 综合测试 ============

#[test]
fn lex_comprehensive_program() {
    let src = "DECLARE X : INTEGER\nX ← 5\nIF X = 5 THEN\n   OUTPUT \"hello\", X\nENDIF\n";
    let kinds = lex_kinds(src, &ParseConfig::normal());

    // 关键断言（不逐一核对，验证关键节点存在）
    assert_eq!(kinds[0], "Kw(Declare)");
    assert!(kinds.iter().any(|k| k == "Kw(If)"));
    assert!(kinds.iter().any(|k| k == "Kw(Then)"));
    assert!(kinds.iter().any(|k| k == "Kw(Output)"));
    assert!(kinds.iter().any(|k| k == "Kw(EndIf)"));
    assert!(kinds.iter().any(|k| k == "Str(\"hello\")"));
    assert_eq!(kinds[kinds.len() - 1], "EOF");
}

#[test]
fn lex_array_declaration() {
    // ARRAY 也是标识符（类型名由 parser 处理）
    let kinds = lex_kinds("DECLARE A : ARRAY[1:10] OF INTEGER", &ParseConfig::normal());
    assert_eq!(kinds, vec![
        "Kw(Declare)", "Ident(A)", "Colon",
        "Ident(ARRAY)", "LBracket", "Int(1)", "Colon", "Int(10)", "RBracket",
        "Ident(OF)", "Ident(INTEGER)",
        "EOF",
    ]);
}

// ============ 错误处理 ============

#[test]
fn lex_illegal_character() {
    // @ 不是合法字符
    let config = ParseConfig::normal();
    let result = Lexer::new("X @ Y", &config).tokenize();
    assert!(result.is_err(), "@ should be illegal");
}

#[test]
fn lex_empty_input() {
    let kinds = lex_kinds("", &ParseConfig::normal());
    assert_eq!(kinds, vec!["EOF"]);
}

#[test]
fn lex_only_whitespace() {
    let kinds = lex_kinds("   \t  \n  ", &ParseConfig::normal());
    // 只有空白和换行
    assert!(kinds.iter().all(|k| k == "EOL" || k == "EOF"));
}

// ============ span 记录 ============

#[test]
fn lex_span_line_column() {
    let src = "X ← 5\nY ← 10";
    let tokens = Lexer::new(src, &ParseConfig::normal()).tokenize().unwrap();
    // 第一行：X 在 (1,1)
    let x_token = &tokens[0];
    assert_eq!(x_token.span.line, 1);
    assert_eq!(x_token.span.column, 1);
    // 第二行：Y 在 (2,1)
    // 找到 Y token（跳过第一行的 EOL）
    let y_token = tokens.iter().find(|t| {
        matches!(&t.kind, TokenKind::Identifier(s) if s == "Y")
    }).expect("should find Y");
    assert_eq!(y_token.span.line, 2);
    assert_eq!(y_token.span.column, 1);
}
