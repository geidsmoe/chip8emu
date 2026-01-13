use std::error::Error;
use std::{env, fs};
use std::path::Path;


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

    let file_path = args[1].clone();
    let bytes: Vec<u8> = fs::read(Path::new(&file_path))?;
    let opcodes: Vec<_>  = bytes.chunks(2).collect();
    // println!("{:X?}", bytes); // prints "[DE, AD, BE, EF]"
    
    for op in opcodes {
        // TODO: better chunking + destructuring
        let [op1, op2] = op else { continue };
        match (*op1, *op2) {
            (0x00, 0xE0) => println!("{:X?} => Clear", op),
            _ => println!("{:X?} => Not implemented", op)
        }
    }

    Ok(())
}
