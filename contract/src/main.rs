#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::URef;
use core::convert::TryInto;

const DATA_ARG_NAME: &str = "keys";
const METHOD_ARG_NAME: &str = "method";
const NAMED_KEY_ARG_NAME: &str = "named-key";

/**
 * pub fn create_lists_if_not_exists - Create an empty list if the named key doesnt exist
 *
 * @return {type}  description
 */
pub fn create_lists_if_not_exists(named_key: &str) {
    //Look for the named key
    match runtime::get_key(named_key) {
        Some(_key) => {}
        None => {
            //If the named key doesnt exist, we create an empty list and save it under the NAMED_KEY_ARG_NAME
            let empty_list: Vec<&str> = Vec::new();
            let key = storage::new_uref(empty_list).into();
            runtime::put_key(named_key, key);
        }
    }
}
/**
 * pub fn remove_all_elements - Reset the list so that no element are in sync
 *
 * @return {type}  description
 */
pub fn remove_all_elements(named_key: &str) {
    //Look for the named key
    match runtime::get_key(named_key) {
        Some(_key) => {
            let empty_list: Vec<&str> = Vec::new();
            let key = storage::new_uref(empty_list).into();
            runtime::put_key(named_key, key);
        }
        None => {}
    }
}

/**
 * pub fn create_or_add_secret_code - Create or update a list of elements
 *
 * @param  {type} secret_code: String secret_code should be "[ID];[VALUE]"
 * @return {type}                     no return type
 */
pub fn create_or_update_secret_code(named_key: &str, secret_code: String) {
    //Look for the named key
    match runtime::get_key(named_key) {
        Some(_key) => {
            //Get the URref of the named key
            let key: URef = _key.try_into().unwrap_or_revert();
            //Get the value of the URef
            let mut existing_secret_codes: Vec<String> =
                storage::read(key).unwrap_or_revert().unwrap_or_revert();
            //Add the content to the existing secret codes
            existing_secret_codes.push(secret_code);
            //Save the list
            storage::write(key, existing_secret_codes);
        }
        None => {}
    }
}

/**
 * pub fn remove_secret_code_if_exists - Remove a list of string from the existing elements. If an element doesnt exist, we ignore the element.
 *
 * @param  {type} values_to_remove: Vec<String> List of string where the content of each line should be "[ID];[VALUE]"
 * @return {type}                               No return value
 */
pub fn remove_secret_code_if_exists(named_key: &str, values_to_remove: Vec<String>) {
    //Look for the named key
    match runtime::get_key(named_key) {
        Some(_key) => {
            //Get the URref of the named key
            let key: URef = _key.try_into().unwrap_or_revert();
            //Get the value of the URef
            let mut existing_secret_codes: Vec<String> =
                storage::read(key).unwrap_or_revert().unwrap_or_revert();

            //If we have secret codes
            if existing_secret_codes.len() > 0 {
                //Loop through each secret code that we want to remove
                for value_to_remove in values_to_remove.into_iter() {
                    //Split the content to get the ID and the VALUE
                    let secret_code_elements: Vec<&str> =
                        value_to_remove.as_str().split(";").collect();
                    //Search if we have the ID in the existing secret codes
                    //If yes, we remove it
                    if let Some(index) = existing_secret_codes
                        .iter()
                        .position(|r| r.contains(secret_code_elements[0]))
                    {
                        existing_secret_codes.remove(index);
                    }
                }
                //Once we are done, we save our content
                storage::write(key, existing_secret_codes);
            }
        }
        None => {}
    }
}

/**
 * Objective : Store a list of String in the blockchain in the named key "blockchain-authenticator"
 *
 * Parameters :
 *
 * keys : should be a list of string where the content of each line should be "[ID];[VALUE]"
 *
 * method : add => Add the list to the current elements
 *          del => remove the list to the current elements
 *
 **/
#[no_mangle]
pub extern "C" fn call() {
    let data: Vec<String> = runtime::get_named_arg(DATA_ARG_NAME);
    let method: String = runtime::get_named_arg(METHOD_ARG_NAME);
    let named_key: String = runtime::get_named_arg(NAMED_KEY_ARG_NAME);

    //We create the named key if it doenst exist
    create_lists_if_not_exists(named_key.as_str());
    //We remove the existing elements
    remove_secret_code_if_exists(named_key.as_str(), runtime::get_named_arg(DATA_ARG_NAME));
    //If the method is "add", we add the elements in parameter to the existing elements
    if method == "add" {
        let values_to_store = data.into_iter();
        for value_to_store in values_to_store {
            create_or_update_secret_code(named_key.as_str(), value_to_store);
        }
    } else if method == "delall" {
        remove_all_elements(named_key.as_str());
    }
}
