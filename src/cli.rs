use clap::Parser;

#[derive(Parser)]
#[command(
    name = "rosetta",
    version,
    about = "A universal data decoder for the command line",
    long_about = "Rosetta inspects opaque strings and explains what they likely are.\n\
        JWTs, Base64, UUIDs, cron, timestamps, URLs, and more — ranked by confidence.\n\n\
        TIPS:\n\
        • Pass data as an argument: rosetta 'eyJhbGciOiJIUzI1NiJ9...'\n\
        • Pipe when stdin is not a terminal: echo SGVsbG8= | rosetta",
    after_help = "EXAMPLES:\n  \
        rosetta '550e8400-e29b-41d4-a716-446655440000'\n  \
        echo 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.e30.sig' | rosetta\n\n\
        ENVIRONMENT:\n  \
        ROSETTA_MAX_INPUT_BYTES  Default stdin cap when --max-input-bytes is omitted."
)]
pub struct Cli {
    #[arg(value_name = "DATA")]
    pub data: Vec<String>,

    #[arg(long = "max-input-bytes")]
    pub max_input_bytes: Option<usize>,
}
