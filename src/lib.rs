use wasm_bindgen::prelude::*;

use serde_json::to_string;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

/// Struct of the parse result.
#[wasm_bindgen(getter_with_clone)]
pub struct ParseResult {
    /// Indicates whether the passed-in sql is valid or not.
    pub success: bool,
    /// The serialzed JSON when succeeded, an error message when failed.
    pub serialized_result: String,
}

/// Parse potentially multiple `sql` statements.
#[wasm_bindgen]
pub fn parse_statements(sql: &str) -> ParseResult {
    let dialect = MySqlDialect {};
    let try_result = Parser::new(&dialect).try_with_sql(sql);
    let mut tokenized_parser = match try_result {
        Ok(parser) =>  parser,
        Err(err) => {
            let serialized_err = to_string(&(err.to_string())).unwrap();
            return ParseResult {
                success: false,
                serialized_result: serialized_err,
            };
        },
    };
    let statements_result = tokenized_parser.parse_statements();
    let statements = match statements_result {
        Ok(s) => s,
        Err(err) => {
            let serialized_err = to_string(&(err.to_string())).unwrap();
            return ParseResult {
                success: false,
                serialized_result: serialized_err,
            };
        }
    };

    let serialzed = to_string(&statements).unwrap();

    ParseResult {
        success: true,
        serialized_result: serialzed,
    }
}
