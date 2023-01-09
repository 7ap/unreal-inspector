use std::io::prelude::*;

#[link(name = "kernel32")]
extern "system" {
    fn AllocConsole() -> bool;
    fn FreeConsole() -> bool;
    fn SetConsoleTitleA(lpConsoleTitle: *const u8) -> bool;
}

pub struct Console;

impl Console {
    pub unsafe fn attach() {
        AllocConsole();
        SetConsoleTitleA(b"unreal-instpector\0".as_ptr());
    }

    pub fn input(prompt: &str) -> String {
        let mut input = String::new();

        write!(std::io::stdout(), "{}", prompt)
            .map_err(|e| e.to_string())
            .unwrap();

        std::io::stdout()
            .flush()
            .map_err(|e| e.to_string())
            .unwrap();

        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())
            .unwrap();

        input
    }

    pub unsafe fn detach() {
        FreeConsole();
    }
}
