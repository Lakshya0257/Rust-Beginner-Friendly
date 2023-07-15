mod input {
    pub fn read_input() {
        println!("Reading user input...");
        // Code for reading input goes here
    }
}

mod networking {
    pub fn connect() {
        println!("Connecting to the network...");
        // Code for establishing a network connection goes here
    }
}

mod data_processing {
    pub fn process_data() {
        println!("Processing data...");
        // Code for data processing goes here
    }
}

mod externalModule; //to import another module

fn main() {
    externalModule::external_module::access();
    input::read_input();
    networking::connect();
    data_processing::process_data();
}

