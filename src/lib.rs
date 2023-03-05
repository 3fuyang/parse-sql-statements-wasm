use wasm_bindgen::prelude::*;

use serde_json::to_string;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

#[wasm_bindgen]
extern {
    // Send serialized result to the client. 
    pub fn post_client(s: &str);
}

#[wasm_bindgen]
pub fn parse_statements(sql: &str) {
    let dialect = MySqlDialect {};
    let try_result = Parser::new(&dialect).try_with_sql(sql);
    let mut tokenized_parser = match try_result {
        Ok(parser) => parser,
        Err(err) => {
            return post_client(&to_string(&(err.to_string())).unwrap());
        },
    };
    let statements_result = tokenized_parser.parse_statements();
    let statements = match statements_result {
        Ok(s) => s,
        Err(err) => {
            return post_client(&to_string(&(err.to_string())).unwrap());
        }
    };

    let serialzed = to_string(&statements).unwrap();

    post_client(&serialzed);
}
