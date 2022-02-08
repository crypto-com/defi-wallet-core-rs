use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

#[no_mangle]
pub extern "C" fn restore_wallet(
    mnemonic: *const c_char,
    passphrase: *const c_char,
    error: *mut i8,
    error_length: i32,
) -> *mut crate::Wallet {
    let mnemonic_str = unsafe { CStr::from_ptr(mnemonic) };
    let mnemonic = mnemonic_str.to_str().unwrap();
    let passphrase_str = unsafe { CStr::from_ptr(passphrase) };
    let passphrase = passphrase_str.to_str().unwrap();
    let wallet_result = crate::restore_wallet(mnemonic.into(), passphrase.into());
    match wallet_result {
        Ok(wallet) => {
            write_string("OK", error, error_length);

            Box::into_raw(wallet)
        }
        Err(message) => {
            write_string(&message.to_string(), error, error_length);
            ptr::null_mut()
        }
    }
}

fn write_string(src: &str, dst_address: *mut i8, dst_address_length: i32) {
    let c_str = CString::new(src).unwrap();
    let c_str_ptr = c_str.as_ptr();

    let mut max_length = std::cmp::min(dst_address_length, c_str.as_bytes().len() as i32);
    unsafe {
        ptr::copy(c_str_ptr, dst_address, max_length as usize);
        let mut last_position = max_length as usize;
        if last_position >= dst_address_length as usize {
            last_position -= 1;
        }
        *dst_address.offset(last_position as isize) = 0;
    }
}

#[no_mangle]
pub extern "C" fn get_address(
    wallet: *mut crate::Wallet,
    out_address: *mut i8,
    out_address_length: i32,
    error: *mut i8,
    error_length: i32,
) -> bool {
    let wallet = unsafe { Box::from_raw(wallet) };
    let address_result = wallet.get_default_address(crate::ffi::CoinType::CryptoOrgMainnet);

    Box::into_raw(wallet);

    match address_result {
        Ok(address) => {
            write_string("OK", error, error_length);

            write_string(&address.to_string(), out_address, out_address_length);
            true
        }
        Err(message) => {
            write_string(&message.to_string(), error, error_length);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn destroy_wallet(wallet: *mut crate::Wallet) {
    unsafe { Box::from_raw(wallet) };
}
