//! 字句解析(Tokenizer)を行うmodです。
//! Tokenに依存しています

use super::token::{Tokens, Token, get_keyword};


/// Tokenizer の構造体
/// 
/// # Struct
/// 
/// * `input` - 入力された値。
/// * `pos` - 現在の入力位置
/// * `pos_next` - 次の入力位置
/// * `pos_char` - 現在読んでいる文字
/// 
#[derive(Debug, Clone)]
pub struct Tokenizer<'a>  {
    input:    &'a str,
    pos:      usize,
    pos_next: usize,
    pos_char: u8,
}

impl<'a> Tokenizer<'a> {

    /// # tokenizerのコンストラクタ
    /// 
    /// ## 引数
    /// 
    /// * `input` - 合算する1つ目の値。
    /// 
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer{
            input,
            pos: 0,
            pos_next: 0,
            pos_char: 0
        };
        tokenizer.read_char();
        return tokenizer;
    }

    /// # トークンの設定
    /// 
    /// ## 引数
    /// 
    /// * `token_type` - トークンのタイプ
    /// * `literal` - リテラル情報
    /// 
    pub fn new_token(token_type: Token, pos_char: u8)-> Tokens {
        Tokens {
            token_type,
            literal: String::from_utf8(vec![pos_char]).unwrap(),
        }
    }

    /// # character reader
    fn read_char(&mut self) {
        if self.pos_next >= self.input.len() {
            self.pos_char = 0;
        } else {
            self.pos_char = self.input.as_bytes()[self.pos_next];
        }
        self.pos = self.pos_next;
        self.pos_next += 1;
    }

    /// # string reader
    fn read_string(&mut self) -> String {
        let pos = self.pos + 1;
        loop {
            self.read_char();
            if self.pos_char == b'"' || self.pos_char == 0 {
                break;
            }
        }
        self.input[pos..self.pos].to_string()
    }

    /// # identifier reader
    fn read_identifier(&mut self) -> String {
        let pos = self.pos;
        while Self::is_letter(&self.pos_char) {
            self.read_char();
        }
        self.input.get(pos..self.pos).unwrap().to_string()
    }

    /// # number reader
    fn read_number(&mut self) -> String {
        let pos = self.pos;
        while Self::is_digit(&self.pos_char) {
            self.read_char();
        }
        self.input.get(pos..self.pos).unwrap().to_string()
    }

    /// # 文字の先読み
    fn peek_char(&mut self) -> u8 {
        if self.pos_next >= self.input.len() {
            return 0
        } else{
            return self.input.as_bytes()[self.pos_next]
        }
    }

    /// # 文字か判定
    /// a ~ z & A ~ Z & _ に含まれるか
    /// ## 引数
    /// 
    /// * `pos_char` - 判定する文字
    /// 
    fn is_letter(pos_char: &u8) -> bool {
        let pos_char = char::from(*pos_char);
        'a' <= pos_char && pos_char <= 'z' || 'A' <= pos_char && pos_char <= 'Z' || pos_char == '_'
    }

    /// # 数値範囲内か判定
    /// 0 ~ 9 に含まれるか
    /// ## 引数
    /// 
    /// * `pos_char` - 判定する文字
    /// 
    fn is_digit(pos_char: &u8) -> bool {
        let pos_char = char::from(*pos_char);
        '0' <= pos_char && pos_char <= '9'
    }

    /// # 空白判断
    fn skip_whitespace(&mut self) {
        while self.pos_char == b' ' || self.pos_char == b'\t' || self.pos_char == b'\n' || self.pos_char == b'\r' {
            self.read_char();
        }
    }

    /// # 次のトークンを参照
    pub fn next_token(&mut self) -> Tokens {
        self.skip_whitespace();
        let token;
        match self.pos_char {
            
            // 算術系
            b'+' => {
                token = Self::new_token(Token::ADD, self.pos_char);
            }
            b'-' => {
                token = Self::new_token(Token::SUB, self.pos_char);
            }
            b'*' => {
                token = Self::new_token(Token::MUL, self.pos_char);
            }
            b'/' => {
                token = Self::new_token(Token::DIV, self.pos_char);
            }

            // 大なり小なり
            b'<' => {
                token = Self::new_token(Token::LT, self.pos_char);
            }
            b'>' => {
                token = Self::new_token(Token::GT, self.pos_char);
            }

            // 括弧系
            b'(' => {
                token = Self::new_token(Token::L_PAREN, self.pos_char);
            }
            b')' => {
                token = Self::new_token(Token::R_PAREN, self.pos_char);
            }
            b',' => {
                token = Self::new_token(Token::COMMA, self.pos_char);
            }
            b'{' => {
                token = Self::new_token(Token::L_BRACE, self.pos_char);
            }
            b'}' => {
                token = Self::new_token(Token::R_BRACE, self.pos_char);
            }
            b'[' => {
                token = Self::new_token(Token::L_BRACKET, self.pos_char);
            }
            b']' => {
                token = Self::new_token(Token::R_BRACKET, self.pos_char);
            }

            // 区切り
            b';' => {
                token = Self::new_token(Token::SEMI_COLON, self.pos_char);
            }
            b':' => {
                token = Self::new_token(Token::COLON, self.pos_char);
            }
            b'"' => {
                token = Tokens {
                    token_type: Token::STRING,
                    literal: self.read_string()
                }
            }

            b'=' => {
                // if peek_char is '=', the literal will be '==',
                if self.peek_char() == b'=' {
                    let curent_position = self.pos;
                    self.read_char();
                    token =  Tokens {//u8は一文字なので直接tokenに入れる。
                        token_type: Token::EQUAL,
                        literal: String::from(&self.input[curent_position..self.pos_next])
                    }
                } else{
                    token = Self::new_token(Token::ASSIGN, self.pos_char);
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    // if peek_char is '=', the literal will be '!=',
                    let curent_position = self.pos;
                    self.read_char();
                    token =  Tokens {
                        token_type: Token::NOT_EQ,
                        literal: String::from(&self.input[curent_position..self.pos_next])
                    }
                } else {
                    token = Self::new_token(Token::NOT, self.pos_char);
                }
            }

            0 => {
                token = Tokens {
                    token_type:  Token::EOF,
                    literal: String::from(""),
                };
            }
            _   => {
                if Self::is_letter(&self.pos_char) {
                    let ident = self.read_identifier();
                    let ident_token = get_keyword(&ident);
                    token =  Tokens {
                        token_type: ident_token,
                        literal: ident
                    };
                    return token
                } else if Self::is_digit(&self.pos_char) {
                    token =  Tokens {
                        token_type: Token::INT,
                        literal: self.read_number()
                    };
                    return token
                } else {
                    token = Self::new_token(Token::ILLEGAL, self.pos_char);
                }
            }
        }
        self.read_char();
        return token;
    }
}
