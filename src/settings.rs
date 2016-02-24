//
// Copyright (c) 2016 IBM Corporation
// Author: Russell Currey <ruscur@russell.cc>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// settings.rs - handle snowpatch settings parsing from TOML
//

use toml;
use toml::{Parser, Value};

use git2::{Repository, Error};

use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;

#[derive(RustcDecodable, Clone)]
pub struct Patchwork {
    pub url: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub pass: Option<String>
}

// TODO: make this CI server agnostic (i.e buildbot or whatever)
#[derive(RustcDecodable, Clone)]
pub struct Jenkins {
    pub url: String,
    pub port: Option<u16>,
    pub token: Option<String>
}

#[derive(RustcDecodable, Clone)]
pub struct Project {
    pub repository: String,
    pub branch: String,
    pub remote_name: String,
    pub remote_uri: String,
    pub jobs: Vec<String>,
    pub push_results: Option<bool>
}

impl Project {
    pub fn get_repo(&self) -> Result<Repository, Error> {
        return Repository::open(&self.repository);
    }
}

#[derive(RustcDecodable, Clone)]
pub struct Config {
    pub patchwork: Patchwork,
    pub jenkins: Jenkins,
    pub projects: BTreeMap<String, Project>
}

pub fn parse(path: String) -> Config {
    let mut toml_config = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Couldn't open config file, exiting.")
    };

    file.read_to_string(&mut toml_config)
        .unwrap_or_else(|err| panic!("Couldn't read config: {}", err));

    let mut parser = Parser::new(&toml_config);
    let toml = parser.parse();

    if toml.is_none() {
        panic!("Syntax error in TOML file, exiting.");
    }

    let config = Value::Table(toml.unwrap());

    match toml::decode(config) {
        Some(t) => t,
        None => panic!("Couldn't deserialize config, exiting.")
    }
}
