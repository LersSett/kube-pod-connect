extern crate clap;

mod cli;

use std::process::{exit, Command};

fn main() {
  let matches = cli::args();
  if matches.is_present("namespaces") {
    get_namespaces()
  }

  if matches.is_present("pode_names") {
    let namespace = matches.value_of("pode_names").unwrap();
    get_pode_names(namespace)
  }

  match matches.value_of("NAMESPACE") {
    Some(namespace) =>
      match matches.value_of("PODE") {
        Some(pode) =>
          match matches.value_of("COMMAND") {
            Some(command) => {
              let result = Command::new("kubectl")
                .arg("-n")
                .arg(namespace)
                .arg("-ti")
                .arg("exec")
                .arg(pode)
                .arg(command)
                .spawn();

              match result {
                Ok(child) => child.wait_with_output().expect("failed to wait on child"),
                Err(error) => panic!("{:?}", error)
              };
            },
            None => panic!("{:?}", "Instance name is required")
          },
        None => panic!("{:?}", "Instance name is required")
      },
    None => exit(0)
  }
}

fn get_namespaces() {
  let result = Command::new("kubectl")
    .arg("get")
    .arg("ns")
    .arg("-o")
    .arg("custom-columns=NAME:.metadata.name")
    .output();

  match result {
    Ok(raw_namespaces) => {
      let mut parsed = std::str::from_utf8(&raw_namespaces.stdout)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
      parsed.drain(0..1);

      println!("{}", cli::Raw(parsed))
    },
    Err(err) => panic!("{:?}", err)
  };
}

fn get_pode_names(namespace: &str) {
  let result = Command::new("kubectl")
    .arg("-n")
    .arg(namespace)
    .arg("get")
    .arg("po")
    .arg("-o")
    .arg("custom-columns=NAME:.metadata.name")
    .output();

  match result {
    Ok(raw_namespaces) => {
      let mut parsed = std::str::from_utf8(&raw_namespaces.stdout)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
      parsed.drain(0..1);

      println!("{}", cli::Raw(parsed))
    },
    Err(err) => panic!("{:?}", err)
  };
}
