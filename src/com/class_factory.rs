use windows::{
    core::{ComInterface, IInspectable, IUnknown, Result, GUID},
    Win32::{
        Foundation::{BOOL, CLASS_E_NOAGGREGATION, E_INVALIDARG, E_POINTER},
        System::Com::IClassFactory_Impl,
    },
};

impl IClassFactory_Impl for crate::PornvirFactory {
    fn CreateInstance(
        &self,
        punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppv: *mut *mut::core::ffi::c_void
    ) -> Result<()>
    {
        if ppv.is_null() {
            return Err(E_POINTER.into());
        }
        unsafe { *ppv = std::ptr::null_mut(); }

        if riid.is_null() {
            return Err(E_INVALIDARG.into());
        }

        if punkouter.is_some() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }

        let instance: IInspectable = crate::Pornvir::new().into();
        unsafe { instance.query(riid, ppv).ok() }
    }

    fn LockServer(&self, _flock: BOOL) ->  Result<()> {
        Ok(())
    }
}