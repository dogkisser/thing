use windows::{
    core::Result,
    Win32::{
        System::Com::IStream,
        UI::Shell::PropertiesSystem::IInitializeWithStream_Impl
    },
};

impl IInitializeWithStream_Impl for crate::Pornvir {
    fn Initialize(&self, pstream: Option<&IStream>, grfmode: u32) -> Result<()> {
        Ok(())
    }
}