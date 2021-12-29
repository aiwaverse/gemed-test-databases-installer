extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;

pub struct Installation {
    pub prod_bases: Vec<String>,
    pub test_bases: Vec<String>,
    pub available_bases: Vec<String>,
    pub single_base: bool,
}

impl Installation {
    pub fn new() -> Installation {
        let single_base;
        let gemed_key = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(r"Software\Interprocess\GemedOnco", KEY_READ)
            .expect("Não foi possível abrir a chave do registro.");
        let clients: String = match gemed_key.get_value("Clientes") {
            Ok(res) => {
                single_base = false;
                res
            }
            Err(_) => {
                single_base = true;
                gemed_key
                    .get_value("Cliente")
                    .expect("Não foi possível encontrar nem a chave cliente, nem a chave clientes.")
            }
        };
        let installed_bases: Vec<String> = clients.split(',').map(|x| x.to_owned()).collect();
        let test_bases: Vec<String> = installed_bases
            .iter()
            .filter(|x| x.to_lowercase().ends_with("_teste"))
            .map(|x| x.to_owned())
            .collect();
        let prod_bases: Vec<String> = installed_bases
            .iter()
            .filter(|b| !test_bases.contains(b))
            .map(|x| x.to_owned())
            .collect();
        let available_bases: Vec<String> = prod_bases
            .iter()
            .filter(|x| !test_bases.contains(&format!("{}_Teste", x)))
            .map(|x| format!("{}_Teste", x))
            .collect();
        Installation {
            prod_bases,
            test_bases,
            available_bases,
            single_base,
        }
    }
}
