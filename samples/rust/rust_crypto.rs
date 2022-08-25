#![allow(missing_docs)]
use kernel::prelude::*;
use kernel::crypto;

module! {
    type: Test,
    name: "rust_crypto",
    author: "Rust for Linux Contributors",
    description: "Rust crypto Sample",
    license: "GPL v2",
}

struct Test;


impl kernel::Module for Test {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("********* Rust Crypto File ***********\n");
       
    crypto::RustCrypto::random();
    //pr_info!("{:?}", &s);
        
        
        Ok(Test)
    }
}