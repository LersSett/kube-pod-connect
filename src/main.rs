extern crate clap;

mod cli;

use std::{
  env, fmt, fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}, process::{Command, Output}, str
};

pub struct Raw<'a>(pub Vec<&'a str>);

impl<'a> fmt::Display for Raw<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut res = String::new();

    for ns in self.0.iter() {
      res.push_str(&format!("{}\n", ns))
    }

    write!(f, "{}", res)
  }
}

fn main() {
  let home_dir = env::home_dir().unwrap_or(Path::new("/root").to_path_buf());
  let kube_exec_dir = home_dir.join(".kube-pod-connect");

  cli::run(kube_exec_dir);
}

fn get_namespaces(kube_exec_dir: &PathBuf, force: bool) {
  if !force && kube_exec_dir.join("namespaces").exists() {
    let contents = read_file(kube_exec_dir.join("namespaces"));

    println!("{}", contents);
  } else {
    let result = Command::new("kubectl")
      .arg("--no-headers=true")
      .arg("-o")
      .arg("custom-columns=NAME:.metadata.name")
      .arg("get")
      .arg("ns")
      .output();

    let namespaces = match result {
      Ok(Output { ref stdout, .. }) => {
        let mut parsed = str::from_utf8(stdout)
          .unwrap()
          .split_whitespace()
          .collect::<Vec<&str>>();
        Raw(parsed)
      },
      Err(error) => panic!("{}", error)
    };

    fs::create_dir_all(kube_exec_dir.clone()).expect("Dir not created");
    write_file(kube_exec_dir.join("namespaces"), format!("{}", namespaces));

    println!("{}", namespaces)
  }
}

fn get_pod_names(namespace: &str) {
  let result = Command::new("kubectl")
    .arg("-n")
    .arg(namespace)
    .arg("-o")
    .arg("custom-columns=NAME:.metadata.name")
    .arg("--field-selector=status.phase=Running")
    .arg("--no-headers=true")
    .arg("get")
    .arg("po")
    .output();

  let pod_names = match result {
    Ok(Output { ref stdout, .. }) => {
      let mut parsed = str::from_utf8(stdout)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
      Raw(parsed)
    },
    Err(error) => panic!("{}", error)
  };
  println!("{}", pod_names);
}

fn read_file(path: PathBuf) -> String {
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  contents
}

fn write_file(path: PathBuf, data: String) {
  match File::create(path) {
    Ok(mut file) =>
      match file.write_all(data.as_bytes()) {
        Ok(_result) => (),
        Err(error) => panic!("{:?}", error)
      },
    Err(error) => panic!("{}", error)
  };
}

fn connect(namespace: &str, pod: &str, command: &str) {
  let result = Command::new("kubectl")
    .arg("-n")
    .arg(namespace)
    .arg("-ti")
    .arg("exec")
    .arg(pod)
    .args(command.split(" "))
    .spawn();

  match result {
    Ok(child) => child.wait_with_output().expect("failed to wait on child"),
    Err(error) => panic!("{}", error)
  };
}
