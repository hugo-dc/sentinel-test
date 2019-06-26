use std::fs::File;
use std::io::Read;

fn inject_metering(code: &[u8]) -> Result<Vec<u8>, parity_wasm::elements::Error> {
    let module = parity_wasm::deserialize_buffer(&code)?;

    let memory_page_cost = 256 * 1024;

    let config = pwasm_utils::rules::Set::default()
        .with_forbidden_floats()
        .with_grow_cost(memory_page_cost);

    let result = match pwasm_utils::inject_gas_counter(module, &config) {
        Ok(output) => output,
        Err(_) => {
            return Err(parity_wasm::elements::Error::Other(
                "Metering injection failed.",
            ));
        }
    };

    parity_wasm::serialize(result)
}

fn main() {
    let mut file = File::open("ewasm_token.wasm").unwrap();
    
    let mut code: Vec<u8> = Vec::new();

    file.read_to_end(&mut code);

    match inject_metering(&code) {
        Ok(output) => {
            println!("success!");
            println!("{:?}", output);
        },
        Err(_) => {
            println!("error!");

        }
    }
}
