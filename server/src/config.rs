use serde::deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub account: Account,
    pub database: Database,
}


#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: String,
    pub loginport: u16,
    pub gameport: u16,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub spawnx: String,
    pub spawny: u16,
    pub spawnz: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}