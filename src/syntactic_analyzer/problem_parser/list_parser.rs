use super::*;

impl<'a> Parser<'a> {
    pub fn parse_args(&'a self) -> Result<Vec<Symbol<'a>>, ParsingError> {
        let mut objects = vec![];
        let mut result = vec![];
        let mut token = self.tokenizer.get_token()?;
        loop {
            while let Token::Identifier(symbol) = token {
                objects.push(symbol);
                token = self.tokenizer.get_token()?;
            }
            match token {
                Token::Punctuator(PunctuationType::Dash) => {
                    // match type
                    let object_type = self.tokenizer.get_token()?;
                    token = self.tokenizer.get_token()?;
                    match object_type {
                        Token::Identifier(t) => {
                            for o in objects {
                                result.push(Symbol::new(o, Some(t)));
                            }
                            objects = vec![];
                        }
                        token => {
                            let error = SyntacticError {
                                expected: format!(
                                    "The type of {}",
                                    objects
                                        .into_iter()
                                        .clone()
                                        .collect::<Vec<&'a str>>()
                                        .join(", ")
                                ),
                                found: token,
                                line_number: self.tokenizer.get_line_number(),
                            };
                            return Err(ParsingError::Syntactic(error));
                        }
                    }
                }
                Token::Punctuator(PunctuationType::RParentheses) => {
                    for o in objects {
                        result.push(Symbol::new(o, None));
                    }
                    return Ok(result);
                }
                token => {
                    let error = SyntacticError {
                        expected: "an identifier".to_string(),
                        found: token,
                        line_number: self.tokenizer.get_line_number(),
                    };
                    return Err(ParsingError::Syntactic(error));
                }
            }
        }
    }
}
