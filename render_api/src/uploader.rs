use anyhow::*;
use wgpu::Extent3d;
use std::{io::Read, path::Path};

pub struct AssetUploader<'a> {
    context: &'a mut imgui::Context,
    renderer: &'a mut imgui_wgpu::Renderer,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> AssetUploader<'a> {
    pub fn ctx(&mut self) -> &mut imgui::Context{
        &mut self.context
    }


    pub fn upload_font<P: AsRef<Path>>(&mut self, path: P, size: f32, font_config: Option<imgui::FontConfig>) -> Result<imgui::FontId> {
        let mut buf = Vec::new();

        let result = std::fs::File::open(path);

        let mut file = match result {
            Ok(file) => file,
            Err(e) => return Err(anyhow!(e.to_string())),
        };

        let result = file.read_to_end(&mut buf);

        match result{
            Ok(_) => {},
            Err(e) => return Err(anyhow!(e.to_string())),
        }

        let font_source = imgui::FontSource::TtfData{ data: &buf, size_pixels: size, config: font_config };

        Ok(self.context.fonts().add_font(&[font_source]))
    }

    pub fn update_font_texture(&mut self){
        self.renderer.reload_font_texture(&mut self.context, &mut self.device, &mut self.queue);
    }

    ///Uploads a texture(image) to the GPU. The use should store the TextureId to call the texture when needed.
    /// @param texture_config: If None is provided, TextureConfig::default() will be used. 
    /// @param data is slice of bytes of a unpack image. Get it as you see fit.
    /// @param size is [width, height] in pixels of the image.
    pub fn upload_texture(&mut self, texture_config: Option<imgui_wgpu::TextureConfig>, data: &[u8], size: [u32;2]) -> imgui::TextureId{
        let tex_config = match texture_config{
            Some(tex_cof) => tex_cof,
            None => imgui_wgpu::TextureConfig{
                size: Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
                ..Default::default()
            },
        };

        let texture = imgui_wgpu::Texture::new(self.device, self.renderer, tex_config);
        texture.write(&self.queue, data, size[0], size[1]);

        self.renderer.textures.insert(texture)
    }

    pub(crate) fn new(ctx: &'a mut imgui::Context,renderer: &'a mut imgui_wgpu::Renderer, device: &'a wgpu::Device , queue: &'a wgpu::Queue, ) -> Self{
        Self{
            context: ctx,
            renderer,
            device,
            queue,
        }
    }
}
