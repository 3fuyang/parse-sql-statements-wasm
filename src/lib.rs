use wasm_bindgen::prelude::*;

use serde_json::to_string;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

#[wasm_bindgen]
pub fn parse_statements(sql: &str) -> String {
    let dialect = MySqlDialect {};
    let try_result = Parser::new(&dialect).try_with_sql(sql);
    let mut tokenized_parser = match try_result {
        Ok(parser) =>  parser,
        Err(err) => {
            let serialized_err = to_string(&(err.to_string())).unwrap();
            return serialized_err;
        },
    };
    let statements_result = tokenized_parser.parse_statements();
    let statements = match statements_result {
        Ok(s) => s,
        Err(err) => {
            let serialized_err = to_string(&(err.to_string())).unwrap();
            return serialized_err;
        }
    };

    let serialzed = to_string(&statements).unwrap();

    return serialzed;
}
