use config::{Config, ConfigError};
use serde::{Deserialize};

// pub fn cfg() {
//     let settings = Config::builder()
//         // Add in `./Settings.toml`
//         .add_source(config::File::with_name("config/db"))
//         .add_source(config::File::with_name("config/server"))
//         // Add in settings from the environment (with a prefix of APP)
//         // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
//         // .add_source(config::Environment::with_prefix("APP"))
//         .build()
//         .unwrap();

//     let r = settings.try_deserialize::<Cfg>().unwrap();
//     println!("{:?}", r);
// }

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerCfg,
    pub mongodb: MongoCfg,
    pub jwt: JwtCfg
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(config::File::with_name("config/settings"))
            .build()?;            
        c.try_deserialize()
    }
}


#[derive(Debug, Deserialize)]
pub struct MongoCfg {
    pub host: String,
    pub port: u16,
    pub database: String
}

#[derive(Debug, Deserialize)]
pub struct ServerCfg {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Deserialize)]
pub struct JwtCfg {
    pub access_secret: String,
    pub refresh_secret: String,
    pub aud: String,
    pub duration: usize
}


