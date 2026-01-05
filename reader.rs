use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;

use scanner::{Scanner, EOF};

use crate::types::MalVal::{Bool, Int, Float, Kwd, List, Nil, Str, Sym};
use crate::types::{error, hash_map, list, vector, MalRet, MalVal};

#[derive(Debug, Clone)]
struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn next(&mut self) -> Result<String, MalVal> {
        self.pos += 1;
        Ok(self
            .tokens
            .get(self.pos - 1)
            .ok_or_else(|| Str("underflow".to_string()))?
            .to_string())
    }
    fn peek(&self) -> Result<String, MalVal> {
        Ok(self
            .tokens
            .get(self.pos)
            .ok_or_else(|| Str("underflow".to_string()))?
            .to_string())
    }
}

fn tokenize(str: &str) -> Vec<String> {
    let mut scanner = Scanner::init(str.as_bytes());
    let mut tokens = vec![];

    loop {
        let tok = scanner.scan();
        if tok == EOF {
            break;
        }
        // Skip comments - scanner already handles them
        tokens.push(scanner.token_text());
    }

    tokens
}

fn unescape_str(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(escaped) = chars.next() {
                match escaped {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    _ => {
                        result.push('\\');
                        result.push(escaped);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

// TODO(jig): consider use isize instead of i64 for integer type

fn read_atom(rdr: &mut Reader) -> MalRet {
    let token = rdr.next()?;
    match &token[..] {
        "nil" => Ok(Nil),
        "false" => Ok(Bool(false)),
        "true" => Ok(Bool(true)),
        _ => {
            // Check if it's an integer (starts with optional - followed by digits)
            if token.chars().all(|c| c.is_ascii_digit() || c == '-')
                && token.parse::<i64>().is_ok() {
                Ok(Int(token.parse::<i64>().unwrap()))
            } else if token.chars().all(|c| c.is_ascii_digit() || c == '-' || c == '.')
                && token.parse::<f32>().is_ok() {
                Ok(Float(token.parse::<f32>().unwrap()))
            } else if token.starts_with('\"') && token.ends_with('\"') {
                // String literal
                Ok(Str(unescape_str(&token[1..token.len() - 1])))
            } else if token.starts_with('\"') {
                error("INCOMPLETE:expected '\"', got EOF")
            } else if let Some(keyword) = token.strip_prefix(':') {
                Ok(Kwd(String::from(keyword)))
            } else {
                Ok(Sym(token.to_string()))
            }
        }
    }
}

fn read_seq(rdr: &mut Reader, end: &str) -> Result<Vec<MalVal>, MalVal> {
    let mut seq: Vec<MalVal> = vec![];
    rdr.next()?;
    loop {
        let token = match rdr.peek() {
            Ok(t) => t,
            Err(_) => return error(&format!("INCOMPLETE:expected '{}', got EOF", end)),
        };
        if token == end {
            break;
        }
        seq.push(read_form(rdr)?);
    }
    let _ = rdr.next();
    Ok(seq)
}

fn read_form(rdr: &mut Reader) -> MalRet {
    let token = rdr.peek()?;
    match &token[..] {
        "'" => {
            let _ = rdr.next();
            Ok(list!(Sym("quote".to_string()), read_form(rdr)?))
        }
        "`" => {
            let _ = rdr.next();
            Ok(list!(Sym("quasiquote".to_string()), read_form(rdr)?))
        }
        "~" => {
            let _ = rdr.next();
            Ok(list!(Sym("unquote".to_string()), read_form(rdr)?))
        }
        "~@" => {
            let _ = rdr.next();
            Ok(list!(Sym("splice-unquote".to_string()), read_form(rdr)?))
        }
        "^" => {
            let _ = rdr.next();
            let meta = read_form(rdr)?;
            Ok(list!(Sym("with-meta".to_string()), read_form(rdr)?, meta))
        }
        "@" => {
            let _ = rdr.next();
            Ok(list!(Sym("deref".to_string()), read_form(rdr)?))
        }
        ")" => error("unexpected ')'"),
        "(" => Ok(list(read_seq(rdr, ")")?)),
        "]" => error("unexpected ']'"),
        "[" => Ok(vector(read_seq(rdr, "]")?)),
        "}" => error("unexpected '}'"),
        "{" => hash_map(read_seq(rdr, "}")?.to_vec()),
        _ => read_atom(rdr),
    }
}

pub fn read_str(str: &str) -> MalRet {
    let tokens = tokenize(str);
    //println!("tokens: {:?}", tokens);
    if tokens.is_empty() {
        return error("no input");
    }
    read_form(&mut Reader { pos: 0, tokens })
}

#[cfg(test)]
mod tests {
    use crate::types::MalVal::{Int, List, Str, Sym};

    #[test]
    fn read_str_simple_sum() {
        match super::read_str("(+ 1 2)") {
            Ok(List(lst, _)) => {
                assert_eq!(lst.len(), 3);
                match &lst[0] {
                    Sym(s) => assert_eq!(s, "+"),
                    _ => panic!("Expected Sym(+)"),
                }
                match &lst[1] {
                    Int(n) => assert_eq!(*n, 1),
                    _ => panic!("Expected Int(1)"),
                }
                match &lst[2] {
                    Int(n) => assert_eq!(*n, 2),
                    _ => panic!("Expected Int(2)"),
                }
            },
            Ok(_) => panic!("Expected List"),
            Err(_) => panic!("rep() returned an error"),
        }
    }

    #[test]
    fn read_str_simple_chr() {
        match super::read_str("(str \"aaa\" \"bbb\")") {
            Ok(List(lst, _)) => {
                assert_eq!(lst.len(), 3);
                match &lst[0] {
                    Sym(s) => assert_eq!(s, "str"),
                    _ => panic!("Expected Sym(+)"),
                }
                match &lst[1] {
                    Str(s) => assert_eq!(s, "aaa"),
                    _ => panic!("Expected Str(aaa)"),
                }
                match &lst[2] {
                    Str(s) => assert_eq!(s, "bbb"),
                    _ => panic!("Expected Str(bbb)"),
                }
            },
            Ok(_) => panic!("Expected List"),
            Err(_) => panic!("rep() returned an error"),
        }
    }
}



// stream_tokenizer

/// Trait for reading characters from a stream (no_std compatible)
pub trait CharReader {
    /// Read the next character. Returns None on EOF or error.
    fn read_char(&mut self) -> Option<char>;
}

// Implementation for any Iterator<Item=char> (no_std)
// This covers str::Chars, String::chars(), and custom iterators
impl<I> CharReader for I
where
    I: Iterator<Item = char>,
{
    fn read_char(&mut self) -> Option<char> {
        self.next()
    }
}

// Helper for creating CharReader from byte streams
// Useful for UART and other byte-based sources
pub struct ByteToCharAdapter<R> {
    reader: R,
}

impl<R> ByteToCharAdapter<R> {
    pub fn new(reader: R) -> Self {
        ByteToCharAdapter {
            reader,
        }
    }
}

impl<R> CharReader for ByteToCharAdapter<R>
where
    R: FnMut() -> Option<u8>,
{
    fn read_char(&mut self) -> Option<char> {
        // Simple UTF-8 decoder
        let first_byte = (self.reader)()?;

        // ASCII fast path
        if first_byte < 0x80 {
            return Some(first_byte as char);
        }

        // Multi-byte UTF-8 (simplified - assumes valid UTF-8)
        let num_bytes = if first_byte & 0b1110_0000 == 0b1100_0000 {
            2
        } else if first_byte & 0b1111_0000 == 0b1110_0000 {
            3
        } else if first_byte & 0b1111_1000 == 0b1111_0000 {
            4
        } else {
            return None; // Invalid UTF-8
        };

        let mut bytes = [first_byte, 0, 0, 0];
        for i in 1..num_bytes {
            bytes[i] = (self.reader)()?;
        }

        core::str::from_utf8(&bytes[..num_bytes])
            .ok()?
            .chars()
            .next()
    }
}

/// Stream tokenizer that yields tokens lazily
pub struct TokenStream<R: CharReader> {
    reader: R,
    buffer: String,
    token_buffer: Vec<String>,
    done: bool,
}

impl<R: CharReader> TokenStream<R> {
    pub fn new(reader: R) -> Self {
        TokenStream {
            reader,
            buffer: String::new(),
            token_buffer: Vec::new(),
            done: false,
        }
    }

    fn fill_buffer(&mut self) {
        // Read all remaining characters into buffer
        while let Some(ch) = self.reader.read_char() {
            self.buffer.push(ch);
        }

        if self.buffer.is_empty() {
            self.done = true;
        } else {
            // Tokenize everything
            self.token_buffer = tokenize(&self.buffer);
            self.token_buffer.reverse(); // So we can pop from the end
            self.buffer.clear();
        }
    }
}

impl<R: CharReader> Iterator for TokenStream<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Return from buffer first
        if let Some(token) = self.token_buffer.pop() {
            return Some(token);
        }

        if self.done {
            return None;
        }

        // Fill buffer and try again
        self.fill_buffer();
        self.token_buffer.pop()
    }
}

/// Stream of MAL expressions
pub struct MalStream<R: CharReader> {
    token_stream: TokenStream<R>,
    token_buffer: Vec<String>,
}

impl<R: CharReader> MalStream<R> {
    pub fn new(reader: R) -> Self {
        MalStream {
            token_stream: TokenStream::new(reader),
            token_buffer: Vec::new(),
        }
    }
}

impl<R: CharReader> Iterator for MalStream<R> {
    type Item = MalRet;

    fn next(&mut self) -> Option<Self::Item> {
        // Collect tokens until we have a complete expression
        loop {
            // Try to parse with current buffer
            if !self.token_buffer.is_empty() {
                let result = read_form(&mut Reader {
                    tokens: self.token_buffer.clone(),
                    pos: 0,
                });

                match result {
                    Ok(val) => {
                        self.token_buffer.clear();
                        return Some(Ok(val));
                    }
                    Err(e) => {
                        // Check if it's an incomplete error
                        if let Str(ref msg) = e {
                            if msg.starts_with("INCOMPLETE:") {
                                // Need more tokens, continue reading
                            } else {
                                // Real error
                                self.token_buffer.clear();
                                return Some(Err(e));
                            }
                        } else {
                            self.token_buffer.clear();
                            return Some(Err(e));
                        }
                    }
                }
            }

            // Get next token
            match self.token_stream.next() {
                Some(token) => self.token_buffer.push(token),
                None => {
                    // No more tokens
                    if self.token_buffer.is_empty() {
                        return None;
                    } else {
                        // Try one last parse
                        let result = read_form(&mut Reader {
                            tokens: self.token_buffer.clone(),
                            pos: 0,
                        });
                        self.token_buffer.clear();
                        return Some(result);
                    }
                }
            }
        }
    }
}