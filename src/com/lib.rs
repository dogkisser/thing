//! Pornvir's DLL component for interfacing with the system via COM
#![allow(
    non_snake_case,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
)]
use std::{
    os::windows::ffi::{OsStrExt, OsStringExt},
    sync::{atomic::{AtomicU64, Ordering}, OnceLock},
    ffi::OsString,
};
use windows::{
    core::implement,
    Win32::{
        UI::Shell::{PropertiesSystem::IInitializeWithStream, IThumbnailProvider},
        Foundation::{BOOL, HINSTANCE, MAX_PATH},
        System::{Com::IClassFactory, LibraryLoader, SystemServices},
    }
};

mod register;
mod thumbnail_provider;
mod initialize_with_stream;
mod class_factory;

// I'd like to avoid using this, but I'm not sure how else to identify the running DLL's HINSTANCE
// as GetModuleHandle returns the HINSTANCE of the executable that loaded the DLL.
pub static PORNVIR_HINSTANCE: OnceLock<HINSTANCE> = OnceLock::new();
static PORNVIR_REF_COUNT: AtomicU64 = AtomicU64::new(0);

#[implement(IClassFactory)]
pub struct PornvirFactory;
#[implement(IThumbnailProvider, IInitializeWithStream)]
pub struct Pornvir {
    bitmap: std::sync::RwLock<Vec<u8>>,
}

// For debugging :3c
#[macro_export]
macro_rules! dialog {
    ($($t:tt)*) => { unsafe {
        windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
            windows::Win32::Foundation::HWND(0),
            &windows::core::HSTRING::from(format!($($t)*)),
            &windows::core::HSTRING::from("Pornvir"),
            windows::Win32::UI::WindowsAndMessaging::MB_OK,
        ); }
    };
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    hinstance: HINSTANCE,
    fdw_reason: u32,
    _: *mut std::ffi::c_void,
) -> BOOL {
    match fdw_reason {
        SystemServices::DLL_PROCESS_ATTACH =>
            PORNVIR_HINSTANCE.set(hinstance).is_ok(),
        _ => false,
    }.into()
}

pub unsafe fn module_filename() -> OsString {
    let hinstance = *crate::PORNVIR_HINSTANCE.get().unwrap();
    let mut dll_path = [0u16; MAX_PATH as usize];
    let len = LibraryLoader::GetModuleFileNameW(hinstance, &mut dll_path[..]) as usize;

    OsString::from_wide(&dll_path[0..len])
}

/// Some functions like RegSetValueExW can take many types of data, including strings, which
/// Windows encodes as UTF-16. Unfortunately, these functions' signatures expect a u8 array.
/// So this function converts an OsString to a u8 array of UTF-16 characters.
/// I'm sure there's a nicer way to do this but I don't know what that is.
pub unsafe fn to_utf16_u8_clusterfuck(s: OsString) -> Vec<u8> {
    let s = s.encode_wide().collect::<Vec<u16>>();
    s.align_to::<u8>().1.to_vec()
}

pub fn increment_refs() {
    PORNVIR_REF_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn decrement_refs() {
    PORNVIR_REF_COUNT.fetch_sub(1, Ordering::Relaxed);
}

pub fn refs() -> u64 {
    PORNVIR_REF_COUNT.load(Ordering::Relaxed)
}

impl Pornvir {
    fn new() -> Self {
        increment_refs();
        Self { bitmap: Vec::new().into(), }
    }
}

impl Drop for Pornvir {
    fn drop(&mut self) {
        decrement_refs();
    }
}