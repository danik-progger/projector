use ::clap::Parser;
use rust::{
    config::{Config, Operation},
    opts::Opts,
    projector::Projector,
};

fn main() -> anyhow::Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut proj = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let value = proj.get_all();
            let value = serde_json::to_string(&value)?;
            println!("{}", value);
        }
        Operation::Print(Some(k)) => {
            proj.get_value(k).map(|x| {
                println!("{}", x);
            });
        }
        Operation::Add(k, v) => {
            proj.set_value(k, v);
            proj.save()?;
        }
        Operation::Remove(k) => {
            proj.remove_value(k);
            proj.save()?;
        }
    }

    return Ok(());
}
