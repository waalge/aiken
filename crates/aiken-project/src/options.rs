use std::path::PathBuf;
use aiken_lang::ast::Tracing;

pub struct Options {
    pub code_gen_mode: CodeGenMode,
    pub tracing: Tracing,
}

pub enum CodeGenMode {
    Test {
        match_tests: Option<Vec<String>>,
        verbose: bool,
        exact_match: bool,
        output_json: Option<PathBuf>,
    },
    Build(bool),
    NoOp,
}
