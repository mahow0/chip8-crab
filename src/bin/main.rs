use chip8_crab::input::run;
use ctrlc;

fn main() {
    // TODO: convert demo into valid input for emulator
    let codes = run();
    
    loop {
        if codes.lock().unwrap().len() > 0 {
            println!("{:?}", codes.lock().unwrap());
            codes.lock().unwrap().clear();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
