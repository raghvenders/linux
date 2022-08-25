
//! Delay functions for operations like sleeping.
//!
//! C he&ader: [`include/linux/delay.h`](../../../../include/crypto/hash.h)

#![allow(unused)]
#![allow(missing_docs)]

use crate::bindings;
use crate::str::CStr;
use crate::prelude::*;
use core::mem::MaybeUninit;
use alloc::boxed::Box;




//RustCrypto
#[derive(Debug)]
pub struct RustCrypto {
    text: *mut u8
}

//RustCrypto
impl RustCrypto{

    //random method
    pub fn random(){
        let str_sha256 = CStr::from_bytes_with_nul(b"drbg_nopr_sha256\0").unwrap();
        let rng = unsafe{bindings::crypto_alloc_rng(str_sha256.as_char_ptr(),0,0)};

        //let mut buf: [u8; 32] = MaybeUninit::uninit_array();

        //let mut buf  = Box::<[u8]>::new_zeroed_slice(32);

        let mut buf = unsafe { MaybeUninit::<[u8; 32]>::uninit().assume_init() };

        let ret = unsafe{ bindings::crypto_rng_get_bytes(rng, *buf.as_mut_ptr() as *mut u8, 32)};

        if ret < 0 {
            pr_info!("**** generation of random numbers failed ***\n");
        }else if ret == 0 {
            pr_info!("**** RNG returned no data ***\n");
        }else{
            pr_info!("RNG returned {} bytes of data\n",ret)
        }
        /*
        
            ret = crypto_rng_get_bytes(rng, buf, len);
            if (ret < 0)
                pr_debug("generation of random numbers failed\n");
            else if (ret == 0)
                pr_debug("RNG returned no data");
            else
                pr_debug("RNG returned %d bytes of data\n", ret);
        */

    }
    
}