#![windows_subsystem = "windows"]
extern crate inputbot;
extern crate uuid;
extern crate clipboard;
extern crate enigo;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use enigo::*;

use inputbot::{KeybdKey::*};

use uuid::Uuid;

fn main() {
    Numrow0Key.bind(move || {
        if LControlKey.is_pressed() && LShiftKey.is_pressed() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

            // get the copy buffer content to reset it after the uuid stuf
            let content = ctx.get_contents();

            // set the content of the copy buffer to to a new uuid v4
            if let Ok(_) = ctx.set_contents(Uuid::new_v4().to_hyphenated().to_string()) {

                let mut enigo = Enigo::new();

                enigo.key_up(Key::Control);
                enigo.key_up(Key::Shift);
                enigo.key_up(Key::Layout('0'));

                enigo.key_down(Key::Control);
                enigo.key_click(Key::Layout('v'));
                enigo.key_up(Key::Control);

                enigo.key_down(Key::Shift);
                enigo.key_down(Key::Control);

                // reset copy buffer back to it's previous state.
                if let Ok(content) = content {
                    let _ = ctx.set_contents(content);
                }
            }
        }
    });

    ::inputbot::handle_input_events();
}
