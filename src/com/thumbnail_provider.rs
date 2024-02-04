use std::io::Cursor;
use image::io::Reader as ImageReader;
use windows::{
    core::Result,
    Win32::{Graphics::Gdi::{CreateBitmap, HBITMAP}, UI::Shell::{IThumbnailProvider_Impl, WTSAT_RGB, WTS_ALPHATYPE}},
};

const IMAGE_DATA: &[u8] = include_bytes!("../../test_data/image.jpg");

impl IThumbnailProvider_Impl for crate::Pornvir {
    fn GetThumbnail(
        &self,
        cx: u32,
        phbmp: *mut HBITMAP,
        pdwalpha: *mut WTS_ALPHATYPE
    ) -> Result<()>
    {
        let image = ImageReader::new(Cursor::new(IMAGE_DATA))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .thumbnail(cx, cx)
            .into_rgba8();

        let width = image.width();
        let height = image.height();

        {
            let mut writer = self.bitmap.write().unwrap();
            *writer = image.into_raw();
        }
        
        let bitmap = unsafe {
            let bytes = self.bitmap.read().unwrap().as_ptr();
            CreateBitmap(width as i32, height as i32, 1, 32, Some(bytes as *const _))
        };

        unsafe { *phbmp = bitmap; }
        unsafe { *pdwalpha = WTSAT_RGB; }
        Ok(())
    }
}