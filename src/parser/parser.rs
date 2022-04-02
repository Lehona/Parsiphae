use crate::parser::errors::{ParsingError, ParsingErrorKind as PEK, Result};
use crate::lexer::{Token, TokenKind};
use crate::types::Declaration;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    progress: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        let tokens = tokens.iter().filter(|tok| match tok.kind {
            TokenKind::Comment(_) => false,
            _ => true,
        }).cloned().collect();

        Parser { tokens, current: 0, progress: 0 }
    }

    pub fn start(&mut self) -> Result<Vec<Declaration>> {
        let mut decls = Vec::new();

        while !self.check(TokenKind::EOF) {
            decls.push(self.global_declaration()?);

            self.consume(TokenKind::Semicolon)?;
        }

        Ok(decls)
    }

    pub fn save_progress(&mut self) {
        self.progress = self.current;
    }

    pub fn progress(&self) -> usize {
        self.progress
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn check(&self, t: TokenKind) -> bool {
        self.tokens[self.current].kind == t
    }

    pub fn current_id(&self) -> usize {
        self.current
    }

    pub fn current(&mut self) -> Result<Token> {
        if self.current >= self.tokens.len() {
            return Err(ParsingError::from_token(
                PEK::ReachedEOF,
                self.tokens.len()-1,
                false,
            ));
        }

        Ok(self.tokens[self.current].clone())
    }

    pub fn current_ref(&self) -> Result<&Token> {
        if self.current >= self.tokens.len() {
            return Err(ParsingError::from_token(
                PEK::ReachedEOF,
                self.tokens.len()-1,
                false,
            ));
        }

        Ok(&self.tokens[self.current])
    }

    pub fn freeze(&self) -> usize {
        self.current
    }

    pub fn restore(&mut self, frozen: usize) {
        self.current = frozen;
    }

    pub fn consume(&mut self, token: TokenKind) -> Result<()> {
        if self.check(token.clone()) {
            self.advance();
            self.save_progress();
        } else {
            return Err(ParsingError::from_token(PEK::ExpectedToken(token), self.current, true));
        }
        Ok(())
    }

    pub fn has_more(&self) -> bool {
        self.current < self.tokens.len()
    }

    pub fn previous(&mut self) -> Result<Token> {
        if self.current == 0 {
            // TODO unlikely to happen but find a better error message
            return Err(ParsingError::from_token(
                PEK::ReachedEOF,
                self.tokens.len()-1,
                false,
            ));
        }

        Ok(self.tokens[self.current - 1].clone())
    }

    pub fn print_upcoming(&self, prefix: &str) {
        let amount = std::cmp::max(5, self.tokens.len() - self.current);
        print!("[{}] Upcoming: ", prefix);
        for t in self.tokens.iter().skip(self.current).take(amount) {
            print!("{:?}  ", t.kind);
        }
        print!("\n");
    }
}
