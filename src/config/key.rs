use device_query::Keycode;

#[derive(Debug, Clone, PartialEq)]
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
    pub keys: Vec<Keycode>,
    pub event: Event
}
