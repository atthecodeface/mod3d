use std::io::Cursor;

use image::{DynamicImage, ImageFormat, ImageReader};

use mod3d_gl::Gl;

pub fn new_of_glb<G: Gl>(
    render_context: &mut G,
    glb: &[u8],
    node_names: &[&str],
) -> Result<mod3d_base::Instantiable<G>, String> {
    fn buf_reader(file: &mut &[u8], byte_length: usize) -> Result<Option<Vec<u8>>, std::io::Error> {
        use std::io::Read;
        let mut buffer = vec![0; byte_length];
        crate::console_log!("file {} {}", file.len(), byte_length);
        file.read_exact(&mut buffer).unwrap();
        Ok(Some(buffer))
    }

    let mut file = glb;
    crate::console_log!("Before call file {} ", file.len());
    let (mut gltf, opt_buffer_0) = mod3d_gltf::glb_load(&mut file, &buf_reader, 16 * 1000 * 1000)
        .map_err(|e| format!("{e:?}"))?;

    let mut od = mod3d_gltf::ObjectData::new(&gltf);
    for n in node_names {
        od.add_object(&gltf, gltf.get_node(n).unwrap());
    }
    od.derive_uses(&gltf);

    let buffers = od
        .gen_byte_buffers(&mut gltf, &mod3d_gltf::buf_parse_fail, opt_buffer_0)
        .map_err(|e| format!("{e:?}"))?;

    let buffer_data = od.gen_buffer_data::<_, _, G>(&|x| &buffers[x]);

    let (buffer_descriptors) = od.gen_descriptors(&gltf, &|x| &buffer_data[x]);

    let (buffer_index_accessors, buffer_data_accessors) =
        od.gen_accessors(&gltf, &|x| &buffer_data[x], &|x| &buffer_descriptors[x]);

    let vertices = od.gen_vertices(&gltf, &|x| &buffer_index_accessors[x], &|x| {
        &buffer_data_accessors[x]
    });

    fn image_load(
        (buffer_index, byte_offset, byte_length): (usize, usize, usize),
        uri_or_type: &str,
        buffers: &[Vec<u8>],
    ) -> Result<DynamicImage, String> {
        eprintln!("Load image {buffer_index} {byte_offset} {byte_length} {uri_or_type}");
        if byte_length == 0 {
            Err(format!("Cannot load image from file yet {uri_or_type}"))
        } else {
            assert!(buffer_index < buffers.len());
            let buffer =
                Cursor::new(&buffers[buffer_index][byte_offset..byte_offset + byte_length]);
            let reader = match uri_or_type {
                "image/jpeg" => ImageReader::with_format(buffer, ImageFormat::Jpeg),
                "image/png" => ImageReader::with_format(buffer, ImageFormat::Png),
                _ => return Err(format!("Unknown image format {uri_or_type}")),
            };
            let image = reader
                .decode()
                .map_err(|e| format!("Failed to parse image buffer: {e}"))?;
            dbg!(&image.color());
            Ok(image)
        }
    }
    let images = od
        .gen_images(&gltf, &|b, u| image_load(b, u, &buffers))
        .map_err(|e| format!("Failed to parse image buffer: {e}"))?;

    fn texture_of_image<'textures, G>(
        image: &'textures image::DynamicImage,
    ) -> mod3d_base::Texture<'textures, G>
    where
        G: Gl,
    {
        let w = image.width() as usize;
        let h = image.height() as usize;
        let bu8 = mod3d_base::BufferElementType::new_int(false, 8);
        let bu16 = mod3d_base::BufferElementType::new_int(false, 16);
        let bf16 = mod3d_base::BufferElementType::float16();
        let (elements_per_data, ele_type) = {
            match image.color() {
                image::ColorType::L8 => (1, bu8),
                image::ColorType::La8 => (2, bu8),
                image::ColorType::Rgb8 => (3, bu8),
                image::ColorType::Rgba8 => (4, bu8),
                image::ColorType::L16 => (1, bu16),
                image::ColorType::La16 => (2, bu16),
                image::ColorType::Rgb16 => (3, bu16),
                image::ColorType::Rgba16 => (4, bu16),
                image::ColorType::Rgb32F => (3, bf16),
                image::ColorType::Rgba32F => (4, bf16),
                _ => (1, bu8),
            }
        };
        let data = image.as_bytes();
        mod3d_base::Texture::new(data, (w, h, 0), ele_type, elements_per_data)
    }
    let textures: Vec<mod3d_base::Texture<G>> =
        od.gen_textures(&gltf, &|i| &images[i], &texture_of_image);
    let materials = od.gen_materials(&gltf);
    let mut obj = od.gen_object(&gltf, &vertices, &textures, &materials);
    obj.analyze();
    Ok(obj.into_instantiable(render_context))
}
