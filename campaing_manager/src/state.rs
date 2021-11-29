use std::{collections::HashMap, io::Read};

use creature::Creature;

pub struct State {
    tex_id: HashMap<usize, imgui::TextureId>,
    font_id: HashMap<usize, imgui::FontId>,

    pub opened: bool,
    pub creatures: Vec<(String, Creature)>,
    pub error_window_opened: bool,    
    pub id: i32,
}

impl State {
    pub fn new() -> Self {
        let vec_monster = crate::helper_functions::load_db();

        Self {
            creatures: vec_monster,
            opened: true,
            id: 0,
            error_window_opened: true,
            tex_id: HashMap::new(),
            font_id: HashMap::new(),
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
        let font_id = uploader.upload_font(path, size, font_config).unwrap();
        self.font_id.insert(key, font_id);
    }

    pub fn font(&self, id: usize) -> imgui::FontId {
        self.font_id.get(&id).unwrap().clone()
    }

    pub fn insert_texture(
        &mut self,
        uploader: &mut render_api::AssetUploader,
        path: &str,
        size: [u32; 2],
        texture_config: Option<render_api::TextureConfig>,
        key: usize,
    ) {
        let tex_id = State::upload_image(uploader, path, size, texture_config);
        self.tex_id.insert(key, tex_id);
    }

    pub fn texture(&self, id: usize) -> imgui::TextureId {
        *self.tex_id.get(&id).clone().unwrap()
    }

    fn upload_image(
        uploader: &mut render_api::AssetUploader,
        path: &str,
        size: [u32; 2],
        texture_config: Option<render_api::TextureConfig>,
    ) -> imgui::TextureId {
        let mut img = Vec::new();
        std::fs::File::open(path)
            .unwrap()
            .read_to_end(&mut img)
            .unwrap();
        let data = image::load_from_memory(&img).unwrap();
        let rgba = data.as_rgba8().unwrap();
        let tex_id = uploader.upload_texture(texture_config, &rgba, size);
        tex_id
    }
}
