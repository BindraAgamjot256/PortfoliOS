use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::{Lazy, Mutex};

static KEYBOARD: Lazy<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> = Lazy::new(|| {
    Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key, 
        HandleControl::Ignore
    ))
});


pub fn get_key(scancode: u8) -> Option<DecodedKey> {
    let mut keyboard = KEYBOARD.lock();
    let key_event = keyboard.add_byte(scancode);
    if let Ok(Some(key_event)) = key_event {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            Some(key)
        }else { None }
    } else { None }
}