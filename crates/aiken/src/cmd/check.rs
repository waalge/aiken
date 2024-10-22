use super::build::{trace_filter_parser, trace_level_parser};
use aiken_lang::{
    ast::{TraceLevel, Tracing},
    test_framework::PropertyTest,
};
use aiken_project::watch::{self, watch_project, with_project};
use rand::prelude::*;
use std::{
    io::{self, IsTerminal},
    path::PathBuf,
    process,
};

#[derive(clap::Args)]
/// Type-check an Aiken project
pub struct Args {
    /// Path to project
    directory: Option<PathBuf>,

    /// Deny warnings; warnings will be treated as errors
    #[clap(short = 'D', long)]
    deny: bool,

    /// Skip tests; run only the type-checker
    #[clap(short, long)]
    skip_tests: bool,

    /// When enabled, also pretty-print test UPLC on failure
    #[clap(long)]
    debug: bool,

    /// When enabled, re-run the command on file changes instead of exiting
    #[clap(long)]
    watch: bool,

    /// An initial seed to initialize the pseudo-random generator for property-tests.
    #[clap(long, value_name = "UINT")]
    seed: Option<u32>,

    /// Maximum number of successful test run for considering a property-based test valid.
    #[clap(long, default_value_t = PropertyTest::DEFAULT_MAX_SUCCESS, value_name="UINT")]
    max_success: usize,

    /// Only run tests if they match any of these strings.
    /// You can match a module with `-m aiken/list` or `-m list`.
    /// You can match a test with `-m "aiken/list.{map}"` or `-m "aiken/option.{flatten_1}"`
    #[clap(short, long)]
    match_tests: Option<Vec<String>>,

    /// This is meant to be used with `--match-tests`.
    /// It forces test names to match exactly
    #[clap(short, long)]
    exact_match: bool,

    /// Environment to build against.
    #[clap(long)]
    env: Option<String>,

    /// Filter traces to be included in the generated program(s).
    ///
    ///   - user-defined:
    ///       only consider traces that you've explicitly defined
    ///       either through the 'trace' keyword of via the trace-if-false
    ///       ('?') operator.
    ///
    ///   - compiler-generated:
    ///       only included internal traces generated by the
    ///       Aiken compiler, for example in usage of 'expect'.
    ///
    ///   - all:
    ///       include both user-defined and compiler-generated traces.
    ///
    /// [default: all]
    #[clap(short = 'f', long, value_parser=trace_filter_parser(), default_missing_value="all", verbatim_doc_comment, alias="filter_traces")]
    trace_filter: Option<fn(TraceLevel) -> Tracing>,

    /// Choose the verbosity level of traces:
    ///
    ///   - silent:
    ///       disable traces altogether
    ///
    ///   - compact:
    ///       only culprit line numbers are shown on failures
    ///
    ///   - verbose:
    ///       enable full verbose traces as provided by the user or the compiler
    ///
    /// [optional]
    #[clap(short, long, value_parser=trace_level_parser(), default_value_t=TraceLevel::Verbose, verbatim_doc_comment)]
    trace_level: TraceLevel,
}

pub fn exec(
    Args {
        directory,
        deny,
        skip_tests,
        debug,
        match_tests,
        exact_match,
        watch,
        trace_filter,
        trace_level,
        seed,
        max_success,
        env,
    }: Args,
) -> miette::Result<()> {
    let mut rng = rand::thread_rng();

    let seed = seed.unwrap_or_else(|| rng.gen());

    let json_output = !io::stdout().is_terminal();

    let result = if watch {
        watch_project(directory.as_deref(), watch::default_filter, 500, |p| {
            p.check(
                skip_tests,
                match_tests.clone(),
                debug,
                exact_match,
                seed,
                max_success,
                match trace_filter {
                    Some(trace_filter) => trace_filter(trace_level),
                    None => Tracing::All(trace_level),
                },
                env.clone(),
                json_output,
            )
        })
    } else {
        with_project(
            directory.as_deref(),
            deny,
            !io::stdout().is_terminal(),
            |p| {
                p.check(
                    skip_tests,
                    match_tests.clone(),
                    debug,
                    exact_match,
                    seed,
                    max_success,
                    match trace_filter {
                        Some(trace_filter) => trace_filter(trace_level),
                        None => Tracing::All(trace_level),
                    },
                    env.clone(),
                    json_output,
                )
            },
        )
    };

    result.map_err(|_| process::exit(1))
}
