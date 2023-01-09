mod console;

use std::thread;

use console::*;

#[link(name = "kernel32")]
extern "system" {
    fn FreeLibraryAndExitThread(hLibModule: usize, dwExitCode: u32) -> !;
}

#[no_mangle]
unsafe extern "system" fn DllMain(module: usize, reason: u32, _: usize) -> isize {
    if reason == 1 {
        thread::spawn(move || -> anyhow::Result<()> {
            Console::attach();

            println!("Hello, world!");

            loop {
                let input = Console::input("> ");
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                let command: Vec<&str> = input.split_whitespace().collect();

                match command[0].to_lowercase().as_str() {
                    "exit" => break,
                    _ => println!("Command \"{}\" not found.", command[0].to_lowercase()),
                }
            }

            Console::detach();

            FreeLibraryAndExitThread(module as _, 0);
        });

        return 1;
    };

    0
}
