use std::{error::Error, path::Path};

use lnk::ShellLink;
use widestring::WideCStr;
use winapi::um::{
    processthreadsapi::{GetStartupInfoW, STARTUPINFOW},
    winbase::STARTF_TITLEISLINKNAME,
};

pub fn get_shortcut_args() -> Result<String, Box<dyn Error>> {
    let mut startup_info = STARTUPINFOW::default();

    unsafe {
        GetStartupInfoW(&mut startup_info as *mut _);
    }

    let was_launched_from_lnk = startup_info.dwFlags & STARTF_TITLEISLINKNAME > 0;

    if !was_launched_from_lnk {
        return Err("Not launched from shortcut".into());
    }

    let lnk_path = unsafe { WideCStr::from_ptr_str(startup_info.lpTitle) };
    let lnk_path = lnk_path.to_string()?;

    let lnk = ShellLink::open(lnk_path).map_err(|e| format!("Error: {:?}", e))?;

    let arguments_in_lnk = lnk.arguments().as_ref().ok_or("Unable to get arguments")?;

    Ok(arguments_in_lnk.clone())
}

pub fn set_shortcut_args(args: &str) -> Result<(), Box<dyn Error>> {
    let mut startup_info = STARTUPINFOW::default();

    unsafe {
        GetStartupInfoW(&mut startup_info as *mut _);
    }

    let was_launched_from_lnk = startup_info.dwFlags & STARTF_TITLEISLINKNAME > 0;

    if !was_launched_from_lnk {
        return Err("Not launched from shortcut".into());
    }

    let lnk_path = unsafe { WideCStr::from_ptr_str(startup_info.lpTitle) };
    let lnk_path = lnk_path.to_string()?;

    let lnk = ShellLink::open(&lnk_path).map_err(|e| format!("{:?}", e))?;

    let target = lnk.relative_path().as_ref().ok_or("No target")?;

    let target_path = Path::new(&lnk_path)
        .parent()
        .ok_or("Unable to get link folder")?
        .join(target);

    let mut new_lnk = ShellLink::new_simple(target_path).map_err(|e| format!("{:?}", e))?;

    new_lnk.set_arguments(Some(args.to_string()));

    new_lnk
        .save(&lnk_path)
        .map_err(|e| format!("Error: {:?}", e))?;

    Ok(())
}
