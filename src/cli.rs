use clap::{App, Arg, ArgMatches};
use connect;
use get_namespaces;
use get_pod_names;
use std::{path::PathBuf, process::exit};

fn get_matches() -> ArgMatches<'static> {
  App::new("kube-exec")
    .version("0.0.1")
    .about("Kubernetes exec to pod")
    .author("Stanislav Lapata <stanislavlapata@gmail.com>")
    .arg(
      Arg::with_name("NAMESPACE")
        .help("Namespace name")
        .index(1)
        .empty_values(false)
    )
    .arg(Arg::with_name("POD").help("Pod name").index(2).empty_values(false))
    .arg(
      Arg::with_name("COMMAND")
        .help("Command for pod")
        .index(3)
        .empty_values(false)
    )
    .arg(
      Arg::with_name("pod_names")
        .short("p")
        .long("pod-names")
        .takes_value(true)
        .empty_values(false)
        .help("Return pod names from namespace")
    )
    .arg(
      Arg::with_name("namespaces")
        .short("n")
        .long("namespaces")
        .conflicts_with("pod_names")
        .help("List namespaces")
    )
    .arg(
      Arg::with_name("force")
        .short("f")
        .long("force")
        .requires_ifs(&[("", "pod_names"), ("", "namespaces")])
        .help("Force update namespace or pod list")
    )
    .get_matches()
}

pub fn run(kube_exec_dir: PathBuf) {
  let matches = get_matches();

  if matches.is_present("namespaces") {
    get_namespaces(&kube_exec_dir, matches.is_present("force"))
  }

  if matches.is_present("pod_names") {
    let namespace = matches.value_of("pod_names").unwrap();
    get_pod_names(&kube_exec_dir, namespace, matches.is_present("force"))
  }

  match matches.value_of("NAMESPACE") {
    Some(namespace) =>
      match matches.value_of("POD") {
        Some(pod) =>
          match matches.value_of("COMMAND") {
            Some(command) => connect(namespace, pod, command),
            None => panic!("command is required")
          },
        None => panic!("Pod name is required")
      },
    None => exit(0)
  }
}
