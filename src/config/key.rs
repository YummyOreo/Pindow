use device_query::Keycode;

#[derive(Debug, Clone)]
pub enum Event {
    OpenApp,
    OpenAppNum(usize),
    AddApp,
    IncementConfig,
    DecrementConfig,
    IncrementSetConfig,
    DecrementSetConfig,
    SetConfigNum(usize),
    DebugClose
}

#[derive(Debug, Clone)]
pub struct Keybind {
    keys: Vec<Keycode>,
    event: Event
}
