mod screen;

use std::error::Error;
use std::{env, fs};
use std::path::Path;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;
use std::thread;


// Run the ROM for 20 cycles to see the IBM logo on the display. If you can see the IBM logo, you are properly interpreting these opcodes:

// 00E0 - Clear the screen
// 6xnn - Load normal register with immediate value
// Annn - Load index register with immediate value
// 7xnn - Add immediate value to normal register
// Dxyn - Draw sprite to screen (un-aligned)
// If you run the ROM for more than 20 cycles, it will enter an endless loop. If that also works as expected, you've also correctly interpreted the jump opcode:

// 1nnn - Jump

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("not enough arguments".into());
    }

    // components
    let mut memory: [u8; 4096] = [0; 4096];
    let mut display: [u8; 8192] = [0; 8192];
    let mut pc: u16 = 0x0200;
    let mut idx_reg: u16 = 0;
    let mut delay_timer: u8 = 0;
    let mut sound_timer: u8 = 0;
    let mut stack: Vec<u16> = Vec::new();
    let mut registers: [u8; 16] = [0; 16];

    // Read ROM into Memory
    let file_path = args[1].clone();
    let bytes: Vec<u8> = fs::read(Path::new(&file_path))?;
    let mut i = 0x0200;
    for b in bytes {
        memory[i] = b;
        i+=1;
    }
    let mut graphics = screen::Graphics::new();
    graphics.clear();
    graphics.set(5, 5);
    
    'running: loop {
        // fetch and increment
        let op1 = memory[pc as usize];
        let op2 = memory[(pc+1) as usize];
        let op = (op1, op2);
        pc += 2;

        // Extract the 4 nibbles and numbers
        let n1 = (op1 >> 4) & 0x0F;   // First nibble (opcode type)
        let n2 = op1 & 0x0F;          // Second nibble (often x)
        let n3 = (op2 >> 4) & 0x0F;   // Third nibble (often y)
        let n4 = op2 & 0x0F;          // Fourth nibble (often n)
        let nn = op2;
        let nnn = ((n2 as u16) << 8) | op2 as u16;

        // execute
        match (n1, n2, n3, n4) {
            (0x0, 0x0, 0xE, 0x0) => {
                graphics.clear();
                graphics.set(5,5);
                println!("{:X?} => Clear", op)
            },
            (0x6, _, _, _) => {
                println!("{:X?} => Load x{} with {}", op, n2, nn);

                registers[n2 as usize] = nn;
            },
            (0xA, _, _, _) => {
                println!("{:X?} => Load idx reg with {}", op, nnn);

                idx_reg = nnn;
            },
            (0x7, _, _, _) => {
                println!("{:X?} => Add value {} to register x{}", op, nn, n4);

                registers[n2 as usize] += nn;
            },
            (0xD, _, _, _) => println!("{:X?} => Draw sprite to screen value", op),
            (0x1, _, _, _) => {
                println!("{:X?} => Jump pc to {}", op, nnn);
                pc = nnn;
            }
            _ => println!("{:X?} => Not implemented", op)
        }
        let mut event_pump = graphics.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // 'running: loop {
    //     let mut event_pump = graphics.sdl_context.event_pump().unwrap();
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             _ => {}
    //         }
    //     }
    // }

    Ok(())
}
