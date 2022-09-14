mod keymap;

use std::env;
use device_query::DeviceState;
use device_query::DeviceEvents;
use rodio::OutputStream;
use exitcode;

fn main() {
    // We expect a file configuration file to be provided
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() < 2 {
        println!("{} [Configuration File]", &args[0]);
        std::process::exit(exitcode::CONFIG);
    }

    let mut keymap = keymap::KeyMapping::new(&args[1]);
    keymap.read_key_sounds();

     // Get a output stream handle to the default physical sound device
     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
     let _device_state = Some(DeviceState::new());
     let _guard = _device_state.unwrap().on_key_down( move |key| {
         println!("Keyboard key down: {:#?}", key);
             
         keymap.play_key_souns(key, &stream_handle);
     });

     loop {}
}
