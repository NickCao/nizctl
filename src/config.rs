use crate::consts;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keymap {
    pub version: u8,
    pub keyboard: String,
    pub keymap: String,
    pub layout: String,
    pub layers: Vec<Vec<String>>,
}

impl Keymap {
    pub fn new(keyboard: String, keymap: Vec<Vec<u8>>) -> Self {
        Self {
            version: 1,
            keyboard,
            keymap: "default".to_string(),
            layout: "LAYOUT".to_string(),
            layers: layers_from_keymap(keymap),
        }
    }
    pub fn encode(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
    pub fn decode(keymap: &str) -> serde_json::Result<Self> {
        serde_json::from_str(keymap)
    }
}

pub fn layers_from_keymap(keymap: Vec<Vec<u8>>) -> Vec<Vec<String>> {
    keymap
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|&keycode| consts::KEY_CODE_NAME[keycode as usize].to_string())
                .collect()
        })
        .collect()
}

pub fn keymap_from_layers(layers: Vec<Vec<String>>) -> Vec<Vec<u8>> {
    layers
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|name| {
                    consts::KEY_CODE_NAME
                        .iter()
                        .position(|x| x == name)
                        .unwrap_or(0)
                        .try_into()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}
