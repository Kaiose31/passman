use std::env;
use std::env::VarError;
use winreg::{enums::*, RegKey};

pub fn insert_account(name: &String, username: &String, password: &String) {
    let mut value = String::new();
    value.push_str(username);
    value.push_str(" ");
    value.push_str(password);
    let mut ac_name = "passman_".to_string();
    ac_name.push_str(name);

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
    env.set_value(ac_name, &value).unwrap();

    // env::set_var(ac_name, value)
}
pub fn delete_account(name: &String) {
    let mut value = String::new();
    value.push_str("passman_");
    value.push_str(name);

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
    env.delete_value(value).unwrap();
}

pub fn get_all() -> Vec<(String, String)> {
    let mut ret: Vec<(String, String)> = Vec::new();
    for (k, v) in env::vars() {
        if k.as_str().starts_with("passman_") {
            ret.append(&mut vec![(k.replace("passman_", ""), v)]);
        }
    }
    ret
}

pub fn get_one(account_name: &String) -> Result<String, VarError> {
    let mut ac_name = "passman_".to_string();
    ac_name.push_str(&account_name);
    env::var(ac_name)
}
