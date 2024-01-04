use std::{fs::File, time::Instant};
use wasmi::{ImportsBuilder, Module, ModuleInstance, NopExternals, RuntimeValue};

extern crate wabt;
extern crate wasmi;

fn load_from_file(filename: &str) -> Module {
    use std::io::prelude::*;
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    Module::from_buffer(buf).unwrap()
}

fn main() {
    let module = load_from_file("examples/test_rust.wasm");
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .run_start(&mut NopExternals)
        .expect("failed to run start");

    let now = Instant::now();
    println!(
        "Result: {:?}",
        instance.invoke_export("main", &[], &mut NopExternals)
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
