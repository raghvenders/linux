
//! Crypto API Random Number and HASH Function
//!
//! C header: [`include/crypto/rng.h`](../../../../include/crypto/hash.h)

#![allow(unused)]
#![allow(missing_docs)]

use crate::bindings;
use crate::str::CStr;
use crate::prelude::*;
use core::mem::MaybeUninit;
use alloc::{boxed::Box, slice};
use crate::c_str;
use core::mem::transmute;
use crate::error;
use alloc::string::String;
use bindings::{shash_desc, u8_};




//RustCrypto
#[derive(Debug)]
pub struct RustCrypto {
    is_registered: bool,
}

//RustCrypto
impl RustCrypto{

    //random method
    pub fn random() -> Result{

        pr_info!("Crypto API: Enter Random Number Generator");

        let str_sha256 = c_str!("drbg_nopr_sha256");

        //Using initialized array than MaybeInit
        let mut chunkbuf = [0; 32];


        let rng = unsafe{bindings::crypto_alloc_rng(str_sha256.as_char_ptr(),0,0)};

        if error::from_kernel_err_ptr(rng).is_err() {
            pr_err!("could not allocate RNG handle for {:?}\n", rng);
            return error::to_result(-1);
        }

        pr_info!("Crypto API: Crypto alloc Rng is succesfull ");
        

        let ret = unsafe {bindings::crypto_rng_get_bytes(rng, chunkbuf.as_mut_ptr(), 32)};
            
        //pr_info!("*** {}  ** {:?}", ret, String::from_utf8(alloc::vec::Vec::from_raw_parts(chunkbuf.as_mut_ptr(), 32, 32)));
        
        

        if ret < 0 {
            pr_info!("generation of random numbers failed : {} ***\n", ret);
        }else if ret == 0 {
            pr_info!("RNG returned no data ***\n");
        }else{
            pr_info!("RNG returned {} bytes of data\n",ret)
        }

        pr_info!("Crypto API: Exit Random Number Generator");
        Ok(())

    }

    pub fn crypto_sha256() -> Result<RustCrypto>{

        pr_info!("Crypto API: Enter crypto_sha256 Generator");
        let str_sha256 = c_str!("sha256");
        
        let sha256 = unsafe { bindings::crypto_alloc_shash(str_sha256.as_char_ptr(),0,0)};

        pr_info!("********* sha256*********** {}  \n",(&mut *sha256).descsize);
        unsafe {
            let x = &(&mut *sha256).base;
            let x1 = x.__crt_alg;

            pr_info!("#### {:?} {:?} {} ###",(&mut *x1).cra_driver_name, (&mut *x1).cra_name, (&mut *x1).cra_flags);
        }
        unsafe{

            //C Uses alloc and layout, I tried using but no idea how to create shash_desc with that
            //let layout = Layout::new::<shash_desc>();
            //layout.size() + 
            //let ptr = alloc(layout);

            //let kr = bindings::krealloc(ptr::null(), layout.size(), bindings::GFP_KERNEL) as *mut u8;

            //let c = *kr;

            
            let mut sd = bindings::shash_desc{
                tfm: sha256,
                __ctx: crate::bindings::__IncompleteArrayField::new(),
            };

            pr_info!("sha Init\n");

            let shs_desc = &mut sd as *mut shash_desc;
            let mut s = crate::bindings::crypto_shash_init(shs_desc);

            pr_info!("SHash Init - {}", s);

            
            let plain_text = c_str!("This is a test");

           // let empty_layout = Layout::new::<u32>();
            //let emp_ptr = alloc(empty_layout);

            pr_info!("sha Update");
            s = crate::bindings::crypto_shash_update(shs_desc,core::mem::transmute(plain_text.as_char_ptr() as *const u8),plain_text.len() as u32);

            pr_info!("SHash update  - {}", s);

           let mut chunkbuf = [0u8; 32];

           let dataptr = &mut chunkbuf as *mut u8_;

           s = crate::bindings::crypto_shash_final(shs_desc, dataptr);

           pr_info!("SHash Final {}, {:?}", s, *dataptr);
          
           let result = CStr::from_char_ptr(core::mem::transmute(dataptr as *const i8));

          pr_info!("Hex for encrypted Buffer {}",result);


          pr_info!("Something Wrong {}",result.as_str_unchecked());


          

          // pr_info!("{:?}",slice::from_raw_parts_mut(dataptr, 32));

           Ok (RustCrypto { is_registered: true})
        }
    }
    
}