mod hosts;
mod utils;

const RESET_FILE_PATH: &str = "
##
# Host Database
#
# localhost is used to configure the loopback interface
# when the system is booting.  Do not change this entry.
##
127.0.0.1		localhost
255.255.255.255		broadcasthost
::1                          localhost
";
const FILE_PATH: &str = "/etc/hosts";
//const LOCALHOST: &str = "127.0.0.1";

// fn main() -> Result<(), std::io::Error> {
fn main() {
    println!("Let's Relax!");
    let arguments: Vec<String> = std::env::args().map(|arg| arg).collect();
    if arguments.len() < 1 {
        panic!("arguments must be defined");
    }
    utils::utils::handle_args(&arguments);
    // let res_flux = utils::utils::execute_flux_cache();
    // if res_flux.is_err() {
    //     return res_flux;
    // }
    // //utils::utils::plan_sleep(sleep_time);
    // return utils::utils::reset_file(RESET_FILE_PATH);
}
