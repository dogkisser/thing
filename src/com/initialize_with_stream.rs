use windows::{
    core::Result,
    Win32::{
        System::Com::IStream,
        UI::Shell::PropertiesSystem::IInitializeWithStream_Impl
    },
};

// This trait impl is necessary for IThumbnailProvider because it naively assumes we care about
// the actual file contents~
impl IInitializeWithStream_Impl for crate::Pornvir {
    fn Initialize(&self, _pstream: Option<&IStream>, _grfmode: u32) -> Result<()> {
        Ok(())
    }
}