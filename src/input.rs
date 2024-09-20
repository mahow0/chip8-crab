use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::sync::{Arc, Mutex};
use crate::cpu::{CPU, HEIGHT, WIDTH, KeyState};
use crate::loader::load_program;

// the scaling factor determining how much we should "blow up" each pixel by
const SCALE : u32 = 20;


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

pub fn q_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::Q)
}
pub fn w_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::W)
}
pub fn e_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::E)
}
pub fn r_pressed(e: &sdl2::EventPump) -> bool {
    e.keyboard_state().is_scancode_pressed(Scancode::R)
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

pub fn binary_to_rgb(color : bool) -> Color {

    match color {
        true => Color::RGB(255, 255, 255),
        false => Color::RGB(0, 0, 0)
    }

}

pub fn draw_screen(vram : &([[bool; HEIGHT]; WIDTH]), canvas : &mut Canvas<Window>) -> () {
    
    //TODO: add some offset from the boundaries of the canvas
    let mut canvas_row : i32 = 0;
    for row in (0..HEIGHT) {
        let mut canvas_col = 0;
        for col in (0..WIDTH) { 
            let pixel = vram[col][row];
            let pixel_color = binary_to_rgb(pixel);

            // Draw a SCALING_FACTOR x SCALING_FACTOR rect at (canvas_col, canvas_row)
            canvas.set_draw_color(pixel_color);
            canvas.fill_rect(Rect::new(canvas_col, canvas_row, SCALE, SCALE));

            // shift canvas_row and canvas_col to correspond to next pixel location
            canvas_col += SCALE as i32;
        }
        canvas_row += SCALE as i32;
    }


}

pub fn get_keystate(e : &sdl2::EventPump) -> KeyState {
    [
        one_pressed(e),
        two_pressed(e),
        three_pressed(e),
        four_pressed(e),
        q_pressed(e),
        w_pressed(e),
        e_pressed(e),
        r_pressed(e),
        a_pressed(e),
        s_pressed(e),
        d_pressed(e),
        f_pressed(e),
        z_pressed(e),
        x_pressed(e),
        c_pressed(e),
        v_pressed(e)
    ]
}

pub fn run() -> Arc<Mutex<Vec<Scancode>>>{
    let codes = Arc::new(Mutex::new(Vec::new()));
    let codes_ext = codes.clone();
    std::thread::spawn(move || {
        let mut input = String::new();
        println!("Provide path to ROM:");
        std::io::stdin().read_line(&mut input).unwrap();
        let path = input.trim();
        let mut cpu = load_program(path).unwrap();   

        let sdl_context = sdl2::init().expect("sdl2 init failed");
        let video_subsystem = sdl_context.video().expect("video subsystem failed");
        let width :u32 = <usize as TryInto<u32>>::try_into(WIDTH).unwrap() * SCALE;
        let height:u32 = <usize as TryInto<u32>>::try_into(HEIGHT).unwrap() * SCALE;

        let window = video_subsystem
            .window("Chip8-Crab", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).expect("window build failed");

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("canvas build failed");

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

            let keystate = get_keystate(&event_pump);
            codes.lock().unwrap().clear();
            for scancode in event_pump.keyboard_state().pressed_scancodes() {
                println!{"SCANCODE: {:?}", scancode}
                codes.lock().unwrap().push(scancode);
            }




            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

            // The rest of the game loop goes here...

            let instr = cpu.fetch();
            let opcode = cpu.decode(instr);
            println!("OPCODE: {:?}", opcode);
            println!("KEYSTATE: {:?}", keystate);
            cpu.execute(opcode, keystate);
            cpu.decr_timers();
            draw_screen(&(cpu.vram), &mut canvas);
            canvas.present();
        };
    });

    codes_ext
}
