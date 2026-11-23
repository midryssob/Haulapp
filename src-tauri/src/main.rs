// Ve vydané verzi na Windows nezobrazovat konzolové okno.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    hauling_pyro_lib::run()
}
