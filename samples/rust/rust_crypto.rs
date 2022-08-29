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
        pr_info!("********* Sample Crypto API Start ***********\n");
       
        let rng_result = crypto::RustCrypto::random();

        pr_info!("*** Result API Call: rng_random : {} ", rng_result.is_ok());

        let crypto_sha256 = crypto::RustCrypto::crypto_sha256();
        
        pr_info!("*** Result Crypto API Call: crypto_sha256 {} ", crypto_sha256.is_ok());

        pr_info!("********* Sample Crypto API End***********\n");
        
        
        Ok(Test)
    }
}