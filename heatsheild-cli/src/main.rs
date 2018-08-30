extern crate heatshield;

#[macro_use]
extern crate quicli;
extern crate uuid;

use quicli::prelude::*;
use uuid::Uuid;

use heatshield::{
    account::{controller::AccountController, model::Account},
    controller::ResourceController,
    salt::controller::SaltController,
    validate::Validator,
};

#[derive(StructOpt, Debug)]
struct AddUser {
    #[structopt(short = "u")]
    username: String,
    #[structopt(short = "p")]
    password: String,
    #[structopt(short = "e")]
    email: String,
}

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::InferSubcommands"))]
enum Opt {
    #[structopt(name = "gensalt", alias = "gs")]
    GenSalt,
    #[structopt(name = "adduser", raw(aliases = r#"&["add"]"#))]
    AddUser(AddUser),
}

main!(|args: Opt| match args {
  Opt::GenSalt => { SaltController.create_salt(); }
  Opt::AddUser(AddUser { username, password, email }) => {
    let account = Account {
        username: Some(username),
        password: Some(password),
        email: Some(email),
        uuid: Some(Uuid::new_v4()),
    };

    account.validate().expect("Invalid account");
    AccountController.create(&account).expect("Unable to create account"); 
    println!("Account created successfully!");
  }
});
