use clap::{load_yaml, App};
use std::str::FromStr;

mod generator;
mod hex;

fn main() {
    /****************** Define cli arguments ******************/
    let yaml = load_yaml!("../resources/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // set verbosity
    let verbose: bool = matches.is_present("verbose");

    /********************** Load hex data *********************/
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
    if verbose {
        println!("Loaded data: {:x?}", data);
    }

    /********************* Setup generator ********************/

    // Read cli arguments
    let dest_address = matches.value_of("ADDRESS").unwrap().to_string();
    let dest_port = matches
        .value_of("PORT")
        .unwrap()
        .parse::<u16>()
        .expect("Invallid port number");
    let local_address: Option<String> = None; // TODO: get local address from arguments
    let local_port: Option<u16> = None; // TODO: get local port from arguments
    let protocol = generator::Protocol::from_str(matches.value_of("protocol").unwrap_or("tcp"))
        .expect("Invallid protocol name");
    let interface: Option<String> = None; // TODO: get interface from arguments

    // Create generator
    let gen = generator::create_generator(
        dest_address,
        dest_port,
        local_address,
        local_port,
        protocol,
        interface,
    );

    if verbose {
        print!(
            "Generator:\n\
            \tDestination address: {}\n\
            \tLocal address: {}\n\
            \tProtocol: {}\n\
            \tInterface: {}\n",
            gen.get_destination_addr(),
            gen.get_local_addr(),
            gen.get_protocol().to_string(),
            gen.get_interface().unwrap_or("undef".to_string())
        );
    }

    /******************** Generate traffic ********************/
    // Get packet count
    let packet_count: i32 = matches
        .value_of("packet-count")
        .unwrap_or("1")
        .parse()
        .expect("No valid packet count");

    // Start generator
    gen.start(data, packet_count);
}
