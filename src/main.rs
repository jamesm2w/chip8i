pub mod interpreter;

fn main() {
    let data = std::fs::read("./roms/IBM Logo.ch8").unwrap();

    let mut interpreter = interpreter::Chip8I::new();
    interpreter.read_rom(data);
    interpreter.print_memdump();

    chip8_base::run(interpreter);
}
