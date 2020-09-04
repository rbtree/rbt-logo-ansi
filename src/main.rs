use std::io::{self, Write};

fn main() -> io::Result<()> {
  let stdout = io::stdout();
  let mut handle = stdout.lock();

  let width = 2;
  let indent = 2;
  let message = format!(
    "
{i}{c}{r}{r}{r}{c}{c}{c}{c}{c}{c}{c}{c}
{i}{c}{r}{r}{r}{b}{w}{b}{w}{r}{r}{r}{c}
{i}{c}{r}{b}{b}{w}{b}{b}{b}{w}{r}{r}{c}
{i}{c}{b}{b}{w}{b}{b}{b}{w}{b}{w}{r}{c}
{i}{c}{c}{b}{b}{b}{w}{b}{b}{b}{b}{w}{c}
{i}{c}{b}{w}{b}{w}{b}{b}{w}{b}{w}{r}{r}
{i}{r}{r}{b}{b}{w}{b}{b}{b}{b}{w}{r}{r}
{i}{r}{r}{r}{b}{b}{b}{b}{b}{w}{b}{w}{c}
{i}{r}{r}{r}{r}{r}{b}{w}{r}{r}{r}{r}{c}
{i}{r}{r}{r}{r}{r}{b}{w}{r}{r}{r}{r}{c}
{i}{r}{r}{r}{r}{r}{b}{w}{r}{r}{r}{r}{c}
{i}{c}{c}{c}{c}{c}{c}{c}{r}{r}{r}{r}{c}
",
    i = format!("{:width$}", "", width = indent),
    c = format!("{:width$}", "", width = width),
    r = format!("[0;5;31;41m{:width$}[0m", "", width = width),
    b = format!("[0;5;30;40m{:width$}[0m", "", width = width),
    w = format!("[0;5;37;47m{:width$}[0m", "", width = width)
  );

  handle.write_all(message.as_bytes())?;

  Ok(())
}
