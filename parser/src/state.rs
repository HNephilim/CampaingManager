use std::{collections::HashMap};
use crate::desc_parser::UiInstruction;


pub struct State {
    tex_id: HashMap<usize, imgui::TextureId>,
    font_id: HashMap<usize, imgui::FontId>,

    pub instruction: Vec<UiInstruction>,
    pub instruction_level: Vec<UiInstruction>,
    pub text_wrap: f32,
    pub with_level: bool,
}

impl State {
    pub fn new() -> Self {
        

        Self {
            instruction: Vec::new(),
            tex_id: HashMap::new(),
            font_id: HashMap::new(),
            instruction_level: Vec::new(),
            text_wrap: 400.,
            with_level: false,
        }
    }
}

impl State {
    pub const COLOR_BONUS: [f32; 4] = [0., 0.5, 0., 1.];
    pub const COLOR_PENALTY: [f32; 4] = [0.5, 0., 0., 1.];
    pub const COLOR_TITLE: [f32; 4] = [0.7, 0.59, 0.190, 1.];

    pub fn print_title(&self, ui: &imgui::Ui, font_id: usize, text: &str){
        let token = ui.push_font(self.font(font_id));
        ui.text_colored(State::COLOR_TITLE, text);
        token.pop();
    }
}

impl State {
    pub fn insert_font(
        &mut self,
        uploader: &mut render_api::AssetUploader,
        path: &str,
        size: f32,
        font_config: Option<imgui::FontConfig>,
        key: usize,
    ) {
        let font_id = uploader.upload_font(path, size, None).unwrap();
        self.font_id.insert(key, font_id);
    }

    pub fn font(&self, id: usize) -> imgui::FontId {
        self.font_id.get(&id).unwrap().clone()
    }

    // pub fn insert_texture(
    //     &mut self,
    //     uploader: &mut render_api::AssetUploader,
    //     path: &str,
    //     size: [u32; 2],
    //     texture_config: Option<imgui_wgpu::TextureConfig>,
    //     key: usize,
    // ) {
    //     let tex_id = State::upload_image(uploader, path, size, texture_config);
    //     self.tex_id.insert(key, tex_id);
    // }

    pub fn texture(&self, id: usize) -> imgui::TextureId {
        *self.tex_id.get(&id).clone().unwrap()
    }

    // fn upload_image(
    //     uploader: &mut render_api::AssetUploader,
    //     path: &str,
    //     size: [u32; 2],
    //     texture_config: Option<imgui_wgpu::TextureConfig>,
    // ) -> imgui::TextureId {
    //     let mut img = Vec::new();
    //     std::fs::File::open(path)
    //         .unwrap()
    //         .read_to_end(&mut img)
    //         .unwrap();
    //     let data = image::load_from_memory(&img).unwrap();
    //     let rgba = data.as_rgba8().unwrap();
    //     let tex_id = uploader.upload_texture(texture_config, &rgba, size);
    //     tex_id
    // }
}
