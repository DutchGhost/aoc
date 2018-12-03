use crate::input;
use std::{fmt::Debug, io::Result};
use structopt::StructOpt;

pub struct Cli<S, F, R>
where
    S: AsRef<str>,
    F: Fn(&str) -> R,
    R: Debug,
{
    year: S,
    day: u8,
    part: u8,
    solution: F,
}

impl<S, F, R> Cli<S, F, R>
where
    S: AsRef<str>,
    F: Fn(&str) -> R,
    R: Debug,
{
    pub fn new(year: S, day: u8, part: u8, solution: F) -> Self {
        Self {
            year,
            day,
            part,
            solution,
        }
    }

    pub fn run(&self) -> Result<()> {
        #[derive(StructOpt, Debug)]
        struct Session {
            #[structopt(short = "c")]
            cookie: Option<String>,
        }

        let session = Session::from_args();

        if let Some(cookie) = session.cookie {
            input::GetInput::write_cookie(&cookie, self.year.as_ref().into())
        } else {
            self.invoke_day()
        }
    }

    fn invoke_day(&self) -> Result<()> {
        let input = input::GetInput::new::<&str>(self.year.as_ref())?;

        let input = input.get_input(self.day)?;
        let result = (self.solution)(&input);

        println!(
            "Year: {}, day: {} part: {}:\nResult: {:?}",
            self.year.as_ref(),
            self.day,
            self.part,
            result
        );

        Ok(())
    }
}

pub fn run<S, R>(year: S, day: u8, part: u8, solution: impl Fn(&str) -> R)
where
    S: AsRef<str>,
    R: Debug,
{
    let year = year.as_ref();
    if let Err(e) = Cli::new(year, day, part, solution).run() {
        println!(
            "Failed to run year {} day {} part {}: {:?}",
            year, day, part, e
        );
    };
}
