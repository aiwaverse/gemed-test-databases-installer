use gemed_test_database_installer::installer::bases::Installation;

fn main() {
    let inst = Installation::new();
    println!("Installed prod bases: {:?}", inst.prod_bases);
    println!("Installed test bases: {:?}", inst.test_bases);
    println!("Available bases: {:?}", inst.available_bases);
}
