use std::thread;

#[link(name = "kernel32")]
extern "system" {
    fn FreeLibraryAndExitThread(hLibModule: usize, dwExitCode: u32) -> !;
}

#[no_mangle]
unsafe extern "system" fn DllMain(module: usize, reason: u32, _: usize) -> isize {
    if reason == 1 {
        thread::spawn(move || {
            // ...

            FreeLibraryAndExitThread(module as _, 0);
        });

        return 1;
    };

    0
}
