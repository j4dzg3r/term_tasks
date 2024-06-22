extern crate termion;

use std::io::{stdin, stdout, Write};

use termion::{
    clear, cursor, event::Key, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen,
    style,
};

fn _cannot_use_stdout() -> &'static str {
    "Cannot use stdout"
}

fn render_start_screen<W>(stdout: &mut W)
where
    W: Write,
{
    write!(
        stdout,
        "{clear}{goto}Welcome to terminal task manager!\r\n\
        If you are green, you always can press <Ctrl+h>\r\n\
        to see available command\r\n\
        - Press <Enter> to run task manager\r\n\
        - Press <q> to quit\r\n",
        clear = clear::All,
        goto = cursor::Goto(1, 1)
    )
    .expect(_cannot_use_stdout());
    stdout.flush().expect(_cannot_use_stdout());
}

fn main() -> Result<(), String> {
    let mut stdin = stdin();
    let mut stdout = stdout()
        .into_raw_mode()
        .expect("Cannot run raw mode")
        .into_alternate_screen()
        .expect("Cannot run alternate screen");
    render_start_screen(&mut stdout);
    for c in stdin.keys() {
        let evt = c.expect("Incorrect events");
        match evt {
            Key::Char('q') => {
                write!(stdout, "{}", style::Reset).expect(_cannot_use_stdout());
                return Ok(());
            }
            Key::Char('\n') => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
