extern crate winreg;
use std::error::Error;
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

    pub fn change_to_single_base(&self) {
        match self.change_to_single_base_perms(KEY_WOW64_64KEY) {
            Ok(_) => {
                println!("Registro 64 bits alterado!");
                self.change_to_single_base_perms(KEY_WOW64_32KEY)
                    .expect("Erro ao alterar registro de 32 bits");
            }
            Err(_) => {
                self.change_to_single_base_perms(KEY_READ)
                    .expect("Erro ao alterar registro de 32 bits");
            }
        }
    }

    fn change_to_single_base_perms(&self, perms: u32) -> Result<(), Box<dyn Error>> {
        let mut install_path = String::new();
        if let true = self.single_base {
            panic!("change_to_single_base_perms chamada com uma instalação com mais de uma base!");
        }
        let gemed_key = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(r"Software\Interprocess\GemedOnco", perms)?;
        for key in gemed_key.enum_keys() {
            if install_path.is_empty() {
                install_path = gemed_key
                    .open_subkey_with_flags(key.as_ref().unwrap(), perms)
                    .unwrap()
                    .get_value("InstallPath")
                    .unwrap();
            }
            let ip_path = format!(
                "Software\\Interprocess\\GemedOnco\\{}",
                key.as_ref()
                    .expect("Unwrap da chave Interprocess mal sucedido")
            );
            RegKey::predef(HKEY_LOCAL_MACHINE)
                .delete_subkey_with_flags(ip_path, perms)
                .expect("Não foi possível deletar as sub-chaves");
            let mds_path = format!(
                "Software\\MDS\\{}",
                key.as_ref().expect("Unwrap da chave MDS mal sucedido")
            );
            RegKey::predef(HKEY_LOCAL_MACHINE)
                .delete_subkey_with_flags(mds_path, perms)
                .expect("Não foi possível deletar as sub-chaves");
        }
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .create_subkey_with_flags(format!("Software\\MDS\\{}", self.prod_bases[0]), perms)?;
        let gemed_onco_key = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(r"Software\Interprocess\GemedOnco", perms)?;
        gemed_onco_key.delete_value("Clientes")?;
        gemed_onco_key.set_value("Cliente", &self.prod_bases[0])?;
        gemed_onco_key.set_value("PathBin", &install_path)?;
        gemed_onco_key.set_value("SistemaNome", &String::from("GemedOncologia"))?;
        gemed_onco_key.set_value(
            "URLServidor",
            &String::from("https://app.interprocess.com.br"),
        )?;
        gemed_onco_key.set_value(
            "URLUpdate",
            &String::from("https://app.interprocess.com.br"),
        )?;
        Ok(())
    }
}
