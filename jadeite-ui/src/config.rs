#![allow(dead_code, unused)]

pub struct Config {}

#[derive(Default)]
pub struct Binding(Option<String>, Option<String>);

impl Binding {
    pub fn new() -> Self {
        Self(None, None)
    }

    pub fn with<'a, K: Into<Option<&'a str>>>(key: K) -> Self {
        let key: Option<&str> = key.into();
        let key = key.map(|s| s.to_owned());
        Self(key, None)
    }
}
pub struct KeyBindings {
    pub up: Binding,
    pub left: Binding,
    pub down: Binding,
    pub right: Binding,
    pub a: Binding,
    pub b: Binding,
    pub x: Binding,
    pub y: Binding,
    pub select: Binding,
    pub start: Binding,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self {
            up: Binding::with("W"),
            left: Binding::with("A"),
            down: Binding::with("S"),
            right: Binding::with("D"),
            a: Binding::with("RIGHT"),
            b: Binding::with("LEFT"),
            x: Binding::with("UP"),
            y: Binding::with("DOWN"),
            select: Binding::with("TAB"),
            start: Binding::with("ENTER"),
        }
    }
}
