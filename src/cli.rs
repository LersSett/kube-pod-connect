use clap::{App, Arg, ArgMatches};
use std::fmt;

pub struct Raw<'a>(pub Vec<&'a str>);

impl<'a> fmt::Display for Raw<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut res = String::new();

    for ns in self.0.iter() {
      res.push_str(&format!("{} ", ns))
    }

    write!(f, "{}", res)
  }
}

pub fn args() -> ArgMatches<'static> {
  App::new("kube-exec")
    .version("0.0.1")
    .about("Kubernetes exec to pode")
    .author("Stanislav Lapata <stanislavlapata@gmail.com>")
    .arg(Arg::with_name("NAMESPACE").help("Namespace").index(1))
    .arg(Arg::with_name("PODE").help("Pode name").index(2))
    .arg(Arg::with_name("COMMAND").help("Command for pode").index(3))
    .arg(
      Arg::with_name("pode_names")
        .short("p")
        .long("pode_names")
        .value_name("NAMESPACE")
        .takes_value(true)
        .help("Return pode names from namespace")
    )
    .arg(
      Arg::with_name("namespaces")
        .short("n")
        .long("namespaces")
        .help("List namespaces")
    )
    .get_matches()
}
