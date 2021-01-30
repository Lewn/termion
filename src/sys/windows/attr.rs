use std::{io};

use super::crossterm_winapi::{ConsoleMode, Handle};
use super::Termios;

pub fn get_terminal_attr() -> io::Result<Termios> {
    let input_console_mode = ConsoleMode::from(Handle::current_in_handle()?);
    let output_console_mode = ConsoleMode::from(Handle::current_out_handle()?);
    let input_mode = input_console_mode.mode()?;
    let output_mode = output_console_mode.mode()?;

    Ok(Termios(input_mode, output_mode))
}

pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    let input_console_mode = ConsoleMode::from(Handle::current_in_handle()?);
    let output_console_mode = ConsoleMode::from(Handle::current_out_handle()?);
    input_console_mode.set_mode(termios.0)?;
    output_console_mode.set_mode(termios.1)?;

    Ok(())
}

pub fn raw_terminal_attr(termios: &mut Termios) {
    // These are copied from the MSDocs.
    // Yes, technically, not the best, but Windows won't change these for obvious reasons.
    // We could link in winapi explicitly, as crossterm_winapi is already doing that, but
    // I feel it just adds a bit too much cruft, when we can just do this.
    //
    // https://docs.microsoft.com/en-us/windows/console/setconsolemode#parameters
    const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
    const ENABLE_LINE_INPUT: u32 = 0x0002;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;
    const RAW_MODE_MASK: u32 = ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT;

    fix_windows_console(termios);

    termios.0 = termios.0 & !RAW_MODE_MASK;
}

pub fn fix_windows_console(termios: &mut Termios) {
    const ENABLE_VIRTUAL_TERMINAL_INPUT : u32 = 0x0200;
    const ENABLE_MOUSE_INPUT : u32 = 0x0010;
    const ENABLE_EXTENDED_FLAGS : u32 = 0x0080;
    const ENABLE_QUICK_EDIT_MODE : u32 = 0x0040;
    const ENABLE_WINDOW_INPUT : u32 = 0x0008;

    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
    const ENABLE_WRAP_AT_EOL_OUTPUT: u32 = 0x0002;
    const DISABLE_NEWLINE_AUTO_RETURN: u32 = 0x0008;

    termios.0 |= ENABLE_VIRTUAL_TERMINAL_INPUT | ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS | ENABLE_WINDOW_INPUT;
    termios.0 &= !ENABLE_QUICK_EDIT_MODE;
    termios.1 |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    termios.1 &= !(ENABLE_WRAP_AT_EOL_OUTPUT | DISABLE_NEWLINE_AUTO_RETURN);
}