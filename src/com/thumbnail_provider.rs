use std::io::Cursor;
use image::io::Reader as ImageReader;
use windows::{
    core::Result,
    Win32::{Graphics::Gdi::{CreateBitmap, HBITMAP}, UI::Shell::{IThumbnailProvider_Impl, WTSAT_UNKNOWN, WTS_ALPHATYPE}},
};

const IMAGE_DATA: &[u8] = include_bytes!("../../test_data/image.jpg");

impl IThumbnailProvider_Impl for crate::Pornvir {
    fn GetThumbnail(
        &self,
        _cx: u32,
        phbmp: *mut HBITMAP,
        pdwalpha: *mut WTS_ALPHATYPE
    ) -> Result<()>
    {
        crate::dialog!("trying to draw thumbnail");
        let image = ImageReader::new(Cursor::new(IMAGE_DATA))
            .with_guessed_format().unwrap().decode().unwrap().into_rgba8();
        let bitmap = image.as_raw();
        let bitmap = unsafe {
            let data_ptr = bitmap.as_ptr() as *const _;
            CreateBitmap(image.width() as i32, image.height() as i32, 1, 8 * 4, Some(data_ptr))
        };

        unsafe { *phbmp = bitmap; }

        std::mem::forget(image);

        unsafe { *pdwalpha = WTSAT_UNKNOWN; }
        Ok(())
    }
}