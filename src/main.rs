use clap::{load_yaml, App};

mod hex;

fn main() {
    // Define cli arguments
    let yaml = load_yaml!("../resources/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Check verbosity
    let verbose: bool = matches.is_present("verbose");

    // Read data from hex file if necessary
    let default_data: Vec<u8> = vec![
        0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, // hello world
    ];
    let mut data: Vec<u8> = default_data.clone();
    if matches.is_present("data") {
        let data_path = matches.value_of("data").unwrap();
        data = match hex::read_from_file(data_path) {
            Ok(d) => d,
            Err(_e) => {
                println!("[Warning] could not parse hexdump file, sending \"hello world\" instead");
                default_data
            }
        };
    }

    // DEBUG: print data
    println!("data: {:x?}", data);

    // TODO: generate traffic
}
