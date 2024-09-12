use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::keyboard::Scancode;
use std::sync::{Arc, Mutex};

pub fn one_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::Num1)
} 
pub fn two_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::Num2)
} 
pub fn three_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::Num3)
} 
pub fn four_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::Num4)
} 
pub fn a_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::A)
}
pub fn s_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::S)
}
pub fn d_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::D)
}
pub fn f_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::F)
}
pub fn z_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::Z)
}
pub fn x_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::X)
}
pub fn c_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::C)
}
pub fn v_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::V)
}

pub fn run() -> Arc<Mutex<Vec<Scancode>>>{
    let codes = Arc::new(Mutex::new(Vec::new()));
    let codes_ext = codes.clone();
    std::thread::spawn(move || {
        let sdl_context = sdl2::init().expect("sdl2 init failed");
        let video_subsystem = sdl_context.video().expect("video subsystem failed");

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).expect("window build failed");

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("canvas build failed");

        //TODO: display pixels representing the internal emulator vram
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().expect("event pump failed");
        loop {
            for event in event_pump.poll_iter() {
                
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        return;
                    } 
                    _ => {}
                }
            }

            codes.lock().unwrap().clear();
            for scancode in event_pump.keyboard_state().pressed_scancodes() {
                codes.lock().unwrap().push(scancode);
            }



            canvas.clear();
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            // The rest of the game loop goes here...
        };
    });

    codes_ext
}
