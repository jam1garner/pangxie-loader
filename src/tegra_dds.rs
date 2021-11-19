use binrw::{io::Cursor, BinRead, BinWrite, BinWriterExt};
use image::{DynamicImage, Pixel};

#[derive(BinRead, BinWrite)]
struct DdsHeader {
    image_data_size: u64, // in bytes
    unk: u32,             // 512
    unk2: u32,            // 0x1
    unk3: u32,            // 0x25, texture format probably
    width: u32,           // 2048
    height: u32,          // 4096
    depth: u32,           // always 1
    mips: u32,            // 1, 2, or 4
    bytes_per_pixel: u32, // always 4
    unk7: u16,            // always 7
    unk8: u16,            // always 1
}

const BYTES_PER_PIXEL: usize = 4; // RGBA

pub(crate) fn from_image(img: DynamicImage) -> Option<Vec<u8>> {
    let mut img = img.into_rgba8();

    for pixel in img.pixels_mut() {
        let channels = pixel.channels_mut();

        let alpha = (channels[3] as f32) / 255.0;

        channels[0] = ((channels[0] as f32) * alpha) as _;
        channels[1] = ((channels[1] as f32) * alpha) as _;
        channels[2] = ((channels[2] as f32) * alpha) as _;
    }

    let (width, height) = (img.width() as usize, img.height() as usize);
    let depth = 1;

    let img_bytes = img.into_raw();
    let swizzled_img = tegra_swizzle::swizzle_block_linear(
        width,
        height,
        depth,
        &img_bytes,
        tegra_swizzle::block_height_mip0(height),
        BYTES_PER_PIXEL,
    )
    .unwrap();

    let (width, height, depth) = (width as _, height as _, depth as _);

    let mut out_vec = Cursor::new(Vec::new());
    out_vec
        .write_le(&DdsHeader {
            image_data_size: swizzled_img.len() as _,
            unk: 512,
            unk2: 1,
            unk3: 0x25,
            width,
            height,
            depth,
            mips: 1,
            bytes_per_pixel: BYTES_PER_PIXEL as _,
            unk7: 7,
            unk8: 1,
        })
        .ok()?;
    out_vec.write_le(&swizzled_img).ok()?;

    Some(out_vec.into_inner())
}
