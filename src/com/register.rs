//! Implements interfaces necessary to register pornvir.
use std::ffi::OsString;
use windows::{
    core::{w, ComInterface, GUID, HRESULT, HSTRING, PCWSTR},
    Win32::{
        Foundation::{CLASS_E_CLASSNOTAVAILABLE, E_POINTER, S_FALSE, S_OK},
        UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED, SHCNF_IDLIST},
        System::{Com::IClassFactory, Registry::*},
    },
};

#[no_mangle]
pub static CLSID_Pornvir: GUID = GUID {
    data1: 0x6969_4201,
    data2: 0x430E,
    data3: 0x4274,
    data4: [0xBB, 0xC8, 0x4f, 0x68, 0x19, 0xf5, 0x29, 0x60],
};
// For convenience.
const CLSID_PORNVIR_STR: &str = "{69694201-430e-4274-bbc8-4f6819f52960}";

/// ? for HRESULTs. Can take multiple expressions.
macro_rules! hresult_ensure {
    ($call:expr) => {
        let ret = $call;

        if let Err(e) = ret {
            crate::dialog!("hresult_ensure: {e:#?}");
            return e.code();
        }
    };
    ($($call:expr),+) => { $( hresult_ensure!($call); )* };
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut std::ffi::c_void,
) -> HRESULT
{
    if *rclsid != CLSID_Pornvir {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    if ppv.is_null() {
        return E_POINTER;
    }
    *ppv = std::ptr::null_mut();

    let factory: IClassFactory = crate::PornvirFactory{}.into();
    factory.query(riid, ppv)
}

#[no_mangle]
/// Perhaps quite significant TODO
pub unsafe extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    S_FALSE
    // if crate::refs() != 0 {
    //     S_FALSE
    // } else {
    //     S_OK
    // }
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllRegisterServer() -> HRESULT {
    let mut hkey = HKEY(0);

    let dll_path = crate::to_utf16_u8_clusterfuck(crate::module_filename());
    let threading_model = crate::to_utf16_u8_clusterfuck(OsString::from("Apartment"));

    let root = &HSTRING::from(format!("Software\\Classes\\CLSID\\{CLSID_PORNVIR_STR}"));

    hresult_ensure![
        RegCreateKeyExW(HKEY_LOCAL_MACHINE, root, 0, PCWSTR::null(), REG_OPTION_NON_VOLATILE,
            KEY_WRITE, None, &mut hkey, None),
        RegCreateKeyExW(hkey, w!("InprocServer32"), 0, PCWSTR::null(), REG_OPTION_NON_VOLATILE,
            KEY_WRITE, None, &mut hkey, None),

        // None sets the (default) key
        RegSetValueExW(hkey, None, 0, REG_SZ, Some(&dll_path)),
        RegSetValueExW(hkey, w!("ThreadingModel"), 0, REG_SZ, Some(&threading_model)),

        RegCloseKey(hkey),

        RegCreateKeyExW(HKEY_LOCAL_MACHINE, root, 0, PCWSTR::null(), REG_OPTION_NON_VOLATILE,
            KEY_WRITE, None, &mut hkey, None)
    ];

    SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    S_OK
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllUnregisterServer() -> HRESULT {
    let root = format!("Software\\Classes\\CLSID\\{CLSID_PORNVIR_STR}");

    // TODO: Error handling should probably be a little more stringent here, while ignoring any
    // errors about the keys not existing.
    _ = RegDeleteKeyW(HKEY_LOCAL_MACHINE, &HSTRING::from(root.clone() + "\\InprocServer32"));
    _ = RegDeleteKeyW(HKEY_LOCAL_MACHINE, &HSTRING::from(root));

    SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    S_OK
}