use std::{
    thread::{self, sleep},
    time::Duration,
};

use windows::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    // windowを出す回数
    for _ in 0..200 {
        thread::spawn(|| -> Result<()> {
            unsafe { new_window()? }
            Ok(())
        });
    }
    sleep(Duration::from_secs(u64::MAX));
    Ok(())
}

unsafe fn new_window() -> Result<()> {
    let event = CreateEventW(None, true, false, None)?;
    SetEvent(event)?;
    WaitForSingleObject(event, 0);
    CloseHandle(event)?;

    loop {
        MessageBoxA(None, s!("Button"), s!("Window"), MB_OK);
    }
}
