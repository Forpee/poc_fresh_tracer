use std::fs::File;
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
    // let wasm_binary: Vec<u8> = wabt::wat2wasm(
    //     r#"
    //         (module
    //             (memory $0 1)
    //             (export "memory" (memory $0))
    //             (export "fibonacci" (func $fibonacci))
    //             (func $fibonacci (; 0 ;) (param $0 i32) (result i32)
    //              (block $label$0
    //               (br_if $label$0
    //                (i32.ne
    //                 (i32.or
    //                  (local.get $0)
    //                  (i32.const 1)
    //                 )
    //                 (i32.const 1)
    //                )
    //               )
    //               (return
    //                (local.get $0)
    //               )
    //              )
    //              (i32.add
    //               (call $fibonacci
    //                (i32.add
    //                 (local.get $0)
    //                 (i32.const -1)
    //                )
    //               )
    //               (call $fibonacci
    //                (i32.add
    //                 (local.get $0)
    //                 (i32.const -2)
    //                )
    //               )
    //              )
    //             )
    //            )
    //         "#,
    // )
    // .expect("failed to parse wat");

    // let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");
    let module = load_from_file("examples/test_rust.wasm");
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .run_start(&mut NopExternals)
        .expect("failed to run start");

    println!(
        "Result: {:?}",
        instance.invoke_export("main", &[], &mut NopExternals)
    );
}
