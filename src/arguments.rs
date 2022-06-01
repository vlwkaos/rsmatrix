use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "", long_about = None)]
pub struct Settings {
  /// Set color of shown characters
  ///
  /// OPTIONS:
  ///     white,
  ///     red,
  ///     blue,
  ///     green,
  ///     magenta,
  ///     cyan,
  ///     yellow,
  ///     random,
  ///     rainbow,
  ///     r,g,b
  #[clap(short, long, default_value_t = String::from("green"))]
  pub color: String,
}

pub fn parse_cli_arguments() -> Settings {
  Settings::parse()
}
