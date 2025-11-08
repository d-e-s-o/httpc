// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::env::args;
use std::env::current_exe;
use std::io::Error;
use std::io::Write as _;
use std::io::stderr;
use std::process::Command;
use std::process::ExitCode;


fn usage() -> ExitCode {
  print!(
    "Usage: {name} <COMMAND>

Commands:
  exec <command..>  Execute one or more commands

Options:
  -h, --help  Print help
",
    name = current_exe().unwrap().display(),
  );
  ExitCode::FAILURE
}

fn main() -> ExitCode {
  if args().any(|arg| &arg == "--help" || &arg == "-h") {
    return usage()
  }
  let mut args = args().skip(1);
  let Some(op) = args.next() else {
    return usage()
  };

  let result = match op.as_ref() {
    "exec" => {
      let cmd = args.next().unwrap_or_default();
      let cmd = args.fold(cmd, |mut cmd, arg| {
        cmd += " ";
        cmd += &arg.to_string();
        cmd
      });
      Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status()
        .map_err(|err| Error::other(format!("failed to run `{cmd}`: {err}")))
        .and_then(|status| {
          if !status.success() {
            Err(Error::other(format!(
              "command `{cmd}` failed with status {status}"
            )))
          } else {
            Ok(())
          }
        })
    },
    _ => return usage(),
  };

  if let Err(err) = result {
    let _result = writeln!(stderr(), "{err}");
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}
