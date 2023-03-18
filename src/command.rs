use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="dictrs")]
#[command(author="outman <pochonlee@gmail.com>")]
#[command(version="v0.0.1")]
#[command(about="A little translate tool in terminator with openai.")]
#[command(arg_required_else_help(true))]
pub struct Args {
    #[arg(short='s')]
    #[arg(help="source language, default English")]
    #[arg(default_value="English")]
    #[arg(long)]
    pub(crate) source: String,

    #[arg(short='t')]
    #[arg(help="target language, default Chinese")]
    #[arg(default_value="Chinese")]
    #[arg(long)]
    pub(crate) target: String,

    #[arg(trailing_var_arg=true)]
    #[arg(help="YOUR CONTENT")]
    pub(crate) content: Vec<String>
}