use anyhow::Context;
use std::path::PathBuf;

use crate::opts::Opts;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> anyhow::Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> anyhow::Result<Self, Self::Error> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow::anyhow!(
                    "add expects 2 arguments, got {}",
                    value.len() - 1
                ));
            }

            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("expect to exist"),
                drain.next().expect("expect to exist"),
            ));
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow::anyhow!(
                    "remove expects 1 arguments, got {}",
                    value.len() - 1
                ));
            }

            let arg = value.pop().expect("expect to exist");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow::anyhow!(
                "remove expects 0 or 1 arguments, got {}",
                value.len() - 1
            ));
        }

        let arg = value.pop().expect("expect to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = std::env::current_dir().context("Unable to get current directory")?;
    let mut loc = PathBuf::from(loc);
    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("Unable to get current directory")?);
}


#[cfg(test)]
mod test {
    use crate::{config::Operation, opts::Opts};

    use super::Config;

    #[test]
    fn print_all() -> anyhow::Result<()> {
        let opts: Config = Opts {
            args: vec![],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));

        return Ok(());
    }

    #[test]
    fn print_key() -> anyhow::Result<()> {
        let opts: Config = Opts {
            args: vec![
                String::from("foo"),
            ],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some(String::from("foo"))));

        return Ok(());
    }

    #[test]
    fn add_key() -> anyhow::Result<()> {
        let opts: Config = Opts {
            args: vec![
                String::from("add"),
                String::from("foo"),
                String::from("bar"),
            ],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Add(String::from("foo"), String::from("bar")));

        return Ok(());
    }

    #[test]
    fn remove_key() -> anyhow::Result<()> {
        let opts: Config = Opts {
            args: vec![
                String::from("rm"),
                String::from("foo"),
            ],
            pwd: None,
            config: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Remove(String::from("foo")));

        return Ok(());
    }
}