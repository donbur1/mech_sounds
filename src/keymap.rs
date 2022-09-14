
use std::str::FromStr;
use std::fs;
use std::collections::HashMap;
use device_query::Keycode;
use rodio::OutputStreamHandle;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, source::Source};
use rodio::source::Buffered;
use std::path::Path;

pub struct KeyMapping {
    key_sounds : HashMap<Keycode, Buffered< Decoder< BufReader<File>> > >,
    config_file : String,
}

impl KeyMapping {

    pub fn new(config_file : &String) -> KeyMapping {

        KeyMapping { 
            key_sounds : HashMap::new(),
            config_file : config_file.clone(),
        }
    }

    pub fn read_key_sounds(&mut self) -> Result<(), String>{
        if self.config_file.is_empty() {
            return Err(String::from("Config path is empty."));
        }

        let file_contents = match fs::read_to_string(&self.config_file) {
            Ok(contents) => contents,
            Err(e) => {
                return Err(format!("Can't read config file. File - {}", self.config_file));
            }
        };

        let parsed_json= json::parse(&file_contents).expect("Failed to parse json config file.");

        let root_key = "keymap";
        if !parsed_json.has_key(root_key) {
            return Err(String::from("Root keymap entry missing."));
        }

        println!("Parsed keymap...");
        for value in parsed_json[root_key].entries() {
            println!("keymap: {} - {} ", value.0, value.1.to_string());
            let mut result = Keycode::from_str(value.0);

            if result.is_err() {
                //Device query version 1.1.1 doesn't have Key0 from_str implemented.
                result = match value.0 {
                    "Key0"          => Ok(Keycode::Key0),
                    "Numpad0"       => Ok(Keycode::Numpad0),
                    "Numpad1"       => Ok(Keycode::Numpad1),
                    "Numpad2"       => Ok(Keycode::Numpad2),
                    "Numpad3"       => Ok(Keycode::Numpad3),
                    "Numpad4"       => Ok(Keycode::Numpad4),
                    "Numpad5"       => Ok(Keycode::Numpad5),
                    "Numpad6"       => Ok(Keycode::Numpad6),
                    "Numpad7"       => Ok(Keycode::Numpad7),
                    "Numpad8"       => Ok(Keycode::Numpad8),
                    "Numpad9"       => Ok(Keycode::Numpad9),
                    "NumpadSubtract"=> Ok(Keycode::NumpadSubtract),
                    "NumpadAdd"     => Ok(Keycode::NumpadAdd),
                    "NumpadDivide"  => Ok(Keycode::NumpadDivide),
                    "NumpadMultiply"=> Err(String::from("test")),//Ok(Keycode::NumpadMultiply),
                    _ => Err(String::from("failed to parse keycode"))
                };
            }

            let key_code = match result {
                Err(err) => {
                    println!("{}", err);
                    continue;
                },
                Ok(keycode) => keycode,
            };
            
            let config_path = Path::new(&self.config_file).parent();

            let config_path_str = match config_path {
                Some(path) => path.to_str().unwrap(),
                None => "./",
            };

            let file = File::open(format!("{}/{}", config_path_str, value.1.to_string()).as_str()).unwrap();

            let file_buffer = BufReader::new(file);

            self.key_sounds.insert(key_code, Decoder::new(file_buffer).unwrap().buffered());
        }

        return Ok(());
    }

    pub fn play_key_souns(&self, key : &Keycode, stream : &OutputStreamHandle) -> Result<(), String> {
        let result = self.key_sounds.get(key);

        if result.is_none() {
            return Err(format!("Failed to find audio for key {}", key));
        }

        let file = self.key_sounds.get(key).unwrap().clone();

        // Play the sound directly on the device
        let samples = file.convert_samples();

        match stream.play_raw(samples) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(format!("Failed to play key sound for key {}", key.to_string())),
        }
    }

}
