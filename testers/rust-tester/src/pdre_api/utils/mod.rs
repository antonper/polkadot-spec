mod crypto;
mod network;
mod storage;
mod child_storage;

pub use crypto::CryptoApi;
pub use network::NetworkApi;
pub use storage::StorageApi;
pub use child_storage::ChildStorageApi;

// Convenience function:
// Gets the Wasm blob which was generated by the `build.rs` script
fn get_wasm_blob() -> Vec<u8> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open("test/testers/rust-tester/target/wasm32-unknown-unknown/release/wasm_blob.wasm")
        .expect("Failed to open wasm blob in target");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("Failed to load wasm blob into memory");
    buffer
}

use substrate_executor::error::Error;
use substrate_executor::WasmExecutor;
use substrate_primitives::testing::KeyStore;
use substrate_primitives::Blake2Hasher;
use substrate_state_machine::TestExternalities as CoreTestExternalities;
use wasmi::RuntimeValue::{self, I32};
use wasmi::MemoryRef;

use std::cell::RefCell;
use std::rc::Rc;

type TestExternalities<H> = CoreTestExternalities<H, u64>;


struct CallWasm<'a> {
    ext: &'a mut TestExternalities<Blake2Hasher>,
    blob: &'a [u8],
    method: &'a str,
    //create_param: Box<FnOnce(&mut dyn FnMut(&[u8]) -> Result<u32, Error>) -> Result<Vec<RuntimeValue>, Error>>,
}

impl<'a> CallWasm<'a> {
    fn new(
        ext: &'a mut TestExternalities<Blake2Hasher>,
        blob: &'a [u8],
        method: &'a str,
    ) -> Self {
        CallWasm {
            ext: ext,
            blob: blob,
            method: method,
        }
    }
    /// Calls the final Wasm Runtime function (this method does not get used directly)
    fn call_into_wasm<F, FR, R>(&mut self, create_param: F, filter_return: FR) -> Result<R, Error> where
		F: FnOnce(&mut dyn FnMut(&[u8]) -> Result<u32, Error>) -> Result<Vec<RuntimeValue>, Error>,
		FR: FnOnce(Option<RuntimeValue>, &MemoryRef) -> Result<Option<R>, Error>
    {
        WasmExecutor::new()
            .call_with_custom_signature(
                self.ext,
                1,
                self.blob,
                self.method,
                create_param,
                filter_return
            )
    }
}

fn with_data_output(data: &[u8], output: &[u8]) -> (
    impl FnOnce(&mut dyn FnMut(&[u8]) -> Result<u32, Error>) -> Result<Vec<RuntimeValue>, Error>,
    u32 // pointer to Wasm memory
) {
    let data_c = data.to_owned();
    let output_c = output.to_owned();
    let mut ptr = 0;

    (
        move |alloc| {
            let data_offset = alloc(&data_c)?;
            let output_offset = alloc(&output_c)?;
            ptr = output_offset as u32;
            Ok(vec![
                I32(data_offset as i32),
                I32(data_c.len() as i32),
                I32(output_offset as i32),
            ])
        },
        ptr
    )
}