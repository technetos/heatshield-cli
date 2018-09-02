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
    client::{controller::ClientController, model::Client}
};

#[derive(StructOpt, Debug)]
struct AddClient {
  #[structopt(short = "n")]
  name: String,
  #[structopt(short = "e")]
  email: String,
}

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
    #[structopt(name = "adduser", alias = "au")]
    AddUser(AddUser),
    #[structopt(name = "addclient", alias = "ac")]
    AddClient(AddClient),
}

main!(|args: Opt| match args {
  Opt::GenSalt => { SaltController.create_salt(); }
  Opt::AddUser(AddUser { username, password, email }) => {
    let mut account = Account {
        username: Some(username),
        password: Some(password),
        email: Some(email),
        uuid: Some(Uuid::new_v4()),
    };

    account.validate().expect("Invalid account");
    account.hash_password();
    AccountController.create(&account).expect("Unable to create account"); 

    println!("Account created successfully!");
  }
  Opt::AddClient(AddClient { name, email }) => {
    let client = ClientController.create(&Client {
        uuid: Uuid::new_v4(),
        name: Some(name),
        email: Some(email),
    }).expect("Invalid client");

    println!("Client created successfully! {}", client.client.uuid);
  }
});
