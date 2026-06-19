use crate::{CpcResult, ParseConfig, Span, lexer::{Keyword, Token, TokenKind, get_keywords_map, get_operators_map}};

pub struct Lexer<'a> {
    input: Vec<char>,
    pos: usize, line: usize, col: usize,
    config: &'a ParseConfig,
    keywords_map: std::collections::HashMap<&'static str, Keyword>,
    operators_map: std::collections::HashMap<&'static str, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, config: &'a ParseConfig) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0, line: 1, col: 1,
            config,
            keywords_map: get_keywords_map(config),
            operators_map: get_operators_map(config),
        }
    }

    pub fn tokenize(&mut self) -> CpcResult<Vec<Token>> {
        let mut tokens = vec![];
        while self.pos < self.input.len() {
            let token = self.next_token()?;
            tokens.push(token);
        }
        tokens.push(Token { kind: TokenKind::EOF, span: self.current_span() });
        Ok(tokens)
    }

    fn next_token(&mut self) -> CpcResult<Token> {
        loop {
            match self.peek_char() {
                Some(' ') | Some('\t') | Some('\r') => { self.advance_char(); }
                Some('\n') => {
                    // 换行是一个 token（语句分隔）
                    let span = self.current_span();
                    self.advance_char();
                    return Ok(Token { kind: TokenKind::EOL, span });
                }
                Some('/') if self.peek_next() == Some('/') => {
                    // 注释：跳到行尾
                    while let Some(c) = self.peek_char() {
                        if c == '\n' { break; }
                        self.advance_char();
                    }
                }
                _ => break,   // 遇到实质字符，退出循环去识别 token
            }
        }

        let span = self.current_span();
        let c = match self.peek_char() {
            Some(c) => c,
            None => {
                return Ok(Token { kind: TokenKind::EOF, span: span });
            }
        };

        match c {
            '0'..='9' => self.lex_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier_or_keyword(),
            '"' => self.lex_string(),
            '\'' => self.lex_char(),
            _ => self.lex_operator_or_unknown(),
        }
    }

    fn parse_escape_sequence(&mut self) -> CpcResult<char> {
        let c = self.peek_char().ok_or(crate::CpcError::Lex {
            span: self.current_span(),
            message: "Unterminated escape sequence".to_string(),
        })?;
        self.advance_char();
        match c {
            'n' => Ok('\n'),
            't' => Ok('\t'),
            'r' => Ok('\r'),
            '\\' => Ok('\\'),
            '"' => Ok('"'),
            '\'' => Ok('\''),
            '0' => Ok('\0'),
            _ => Err(crate::CpcError::Lex {
                span: self.current_span(),
                message: format!("Invalid escape character: \\{}", c),
            }),
        }
    }

    fn lex_char(&mut self) -> CpcResult<Token> {
        let span = self.current_span();
        self.advance_char(); // 跳过开头的单引号
        let char_value = match self.peek_char() {
            Some('\\') => {
                self.advance_char(); // 跳过反斜杠
                self.parse_escape_sequence()?
            },
            Some(c) => {
                self.advance_char(); // 跳过字符
                c
            },
            None => {
                return Err(crate::CpcError::Lex {
                    span: self.current_span(),
                    message: "Unterminated character literal".to_string(),
                });
            }
        };
        if self.peek_char() != Some('\'') {
            return Err(crate::CpcError::Lex {
                span: self.current_span(),
                message: "Unterminated character literal".to_string(),
            });
        }
        self.advance_char(); // 跳过结尾的单引号
        Ok(Token { kind: TokenKind::Char(char_value), span })
    }

    fn lex_identifier_or_keyword(&mut self) -> CpcResult<Token> {
        let start_pos = self.pos;
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance_char();
            } else {
                break;
            }
        }
        let end_pos = self.pos;
        let span = Span { line: self.line, column: self.col - (end_pos - start_pos) };
        let ident_str: String = self.input[start_pos..end_pos].iter().collect();
        let upper = ident_str.to_uppercase(); // 关键字始终不敏感，始终使用大写判断
        // 先检查是否为字符操作符
        if let Some(op_kind) = self.operators_map.get(upper.as_str()) {
            return Ok(Token { kind: op_kind.clone(), span });
        }
        // 再检查是不是关键字
        if let Some(keyword_kind) = self.keywords_map.get(upper.as_str()) {
            return Ok(Token { kind: TokenKind::Keyword(*keyword_kind), span });
        }

        let ident_final = if self.config.case_sensitive {
            ident_str                          // 敏感：原样
        } else {
            ident_str.to_lowercase()           // 不敏感：转小写（scope 存小写，比较时一致）
        };
        Ok(Token { kind: TokenKind::Identifier(ident_final), span })
    }

    fn lex_string(&mut self) -> CpcResult<Token> {
        let span = self.current_span();
        self.advance_char(); // 跳过开头的引号
        let mut s = String::new();
        while let Some(c) = self.peek_char() {
            match c {
                '"' => break, // 遇到结尾引号，结束
                '\\' => {
                    self.advance_char(); // 跳过反斜杠
                    s.push(self.parse_escape_sequence()?); // 跳过转义字符（将由下面自动获取并处理）
                },
                '\n' => {
                    if self.config.allow_new_line_in_string {
                        self.advance_char();
                        s.push('\n');
                    } else {
                        return Err(crate::CpcError::Lex {
                            span: self.current_span(),
                            message: "Unterminated string literal".to_string(),
                        });
                    }
                },
                _ => {
                    self.advance_char();
                    s.push(c);
                }
            }
        }
        if self.peek_char() != Some('"') {
            return Err(crate::CpcError::Lex {
                span: self.current_span(),
                message: "Unterminated string literal".to_string(),
            });
        }
        self.advance_char(); // 跳过结尾的引号
        Ok(Token { kind: TokenKind::Str(s), span })
    }

    fn lex_operator_or_unknown(&mut self) -> CpcResult<Token> {
        let span = self.current_span();
        let mut op_str = String::new();
        while let Some(c) = self.peek_char() {
            op_str.push(c);
            if self.operators_map.contains_key(op_str.as_str()) {
                self.advance_char();
            } else {
                op_str.pop(); // 回退最后一个字符
                break;
            }
        }
        if op_str.is_empty() {  // 如果没有匹配到任何字符，说明这个符号不存在
            let bad_char = self.peek_char().unwrap_or('?');
            self.advance_char();   // ← 关键：必须前进，否则潜在死循环
            return Err(crate::CpcError::Lex {
                span: self.current_span(),
                message: format!("Illegal character: '{}'", bad_char),
            });
        }
        if let Some(op_kind) = self.operators_map.get(op_str.as_str()) {
            Ok(Token { kind: op_kind.clone(), span })
        } else {
            Err(crate::CpcError::Lex {
                span,
                message: format!("Unknown operator: {}", op_str),
            })
        }
    }

    fn lex_number(&mut self) -> CpcResult<Token> {
        let span = self.current_span();
        let start_pos = self.pos;   // 循环前的位置
        let mut has_dot = false;
        while let Some(c) = self.peek_char() {
            if c.is_digit(10) {
                self.advance_char();
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.advance_char();
            } else {
                break;
            }
        }
        let num_str: String = self.input[start_pos..self.pos].iter().collect();
        if has_dot {
            let value: f64 = match num_str.parse() {
                Ok(v) => v,
                Err(_) => {
                    return Err(crate::CpcError::Lex {
                        span,
                        message: "Invalid float number".to_string(),
                    });
                }
            };
            Ok(Token { kind: TokenKind::Float(value), span })
        } else {
            let value: i64 = match num_str.parse() {
                Ok(v) => v,
                Err(_) => {
                    return Err(crate::CpcError::Lex {
                        span,
                        message: "Invalid integer number".to_string(),
                    });
                }
            };
            Ok(Token { kind: TokenKind::Int(value), span })
        }
    }

    fn current_span(&self) -> Span {
        Span { line: self.line, column: self.col }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.pos).cloned()
    }

    fn peek_next(&self) -> Option<char> {
        self.input.get(self.pos + 1).cloned()
    }

    fn advance_char(&mut self) {
        if let Some(c) = self.peek_char() {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
    }
}