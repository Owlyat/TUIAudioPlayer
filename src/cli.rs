use clap::Subcommand;

#[derive(Debug, Clone, clap::Parser)]
#[command(
    author = "Owlyat",
    version = "0.1",
    about = "A Terminal User Interface for playing audio"
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    #[clap(short = 'd')]
    debug: bool,
}

impl Cli {
    pub fn get_command(&self) -> Command {
        self.command.clone()
    }
    pub fn get_debug(&self) -> bool {
        self.debug
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Play {
        path: std::path::PathBuf,
        #[clap(short = 'L', long = "lowpass")]
        low_pass: Option<u32>,
        #[clap(short = 'H', long = "highpass")]
        high_pass: Option<u32>,
    },
    Player {
        #[clap(short = 'c', long = "CurrentWorkingDirectory")]
        cwd: Option<std::path::PathBuf>,
    },
    TagWritter {
        path: std::path::PathBuf,
        #[clap(short = 't', long = "Title")]
        title: Option<String>,
        #[clap(short = 'A', long = "Artist")]
        artist: Option<String>,
        #[clap(short = 'a', long = "Album")]
        album: Option<String>,
        #[clap(short = 'g', long = "Genre")]
        genre: Option<String>,
    },
}
