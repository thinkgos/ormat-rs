use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(version,author,about,long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
#[non_exhaustive] // 表明未来还有其它元素添加
pub enum Action {
    /// generate enum json config.
    Enum,
}
