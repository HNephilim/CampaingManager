mod print;
use print::Print;


mod helper_functions;
mod state;

use state::State;

fn main() {
    
    let mut state = State::new();

    let mut executor = render_api::Executor::default();

    let config: render_api::Config<State> = render_api::Config{
        window_title: "Rot's Campaing Manager".to_string(),
        initial_window_width: 1280.,
        initial_window_height: 720.,        
        font_size: Some(15.),
        ..Default::default()
    };



    executor.build(&config, &mut state, |mut uploader, state|{
        
        
        
        state.insert_font(&mut uploader, "assets/fonts/Rubik-Light.ttf", 15., None, 0);
        state.insert_font(&mut uploader, "assets/fonts/Rubik-Bold.ttf", 15., None, 1);        
        
        state.insert_texture(&mut uploader, "assets/imgs/FreeAction_W.png", [75, 75], None, 0);        
        state.insert_texture(&mut uploader, "assets/imgs/OneAction_W.png", [75, 75], None, 1);
        state.insert_texture(&mut uploader, "assets/imgs/TwoActions_W.png", [75, 75], None, 2);
        state.insert_texture(&mut uploader, "assets/imgs/ThreeActions_W.png", [75, 75], None,3);
        state.insert_texture(&mut uploader, "assets/imgs/OneTwoActions_W.png", [75, 75], None,4);
        state.insert_texture(&mut uploader, "assets/imgs/OneThreeActions_W.png", [75, 75], None,5);
        state.insert_texture(&mut uploader, "assets/imgs/Reaction_W.png", [75, 75], None, 6);
       
        uploader.ctx().style_mut().frame_rounding = 12.;
        uploader.ctx().style_mut().window_rounding = 12.;
        uploader.ctx().style_mut().child_rounding = 12.;
        uploader.ctx().style_mut().popup_rounding = 12.;
        uploader.ctx().style_mut()[imgui::StyleColor::WindowBg] = [0.04, 0.04, 0.04 ,1.];
        uploader.ctx().style_mut()[imgui::StyleColor::PopupBg] = [0.1, 0.1, 0.1 ,1.];
        
    });

    executor.run(config, state, |ui, state| {

        let token = ui.push_font(state.font(0));
        

        ui.show_demo_window(&mut state.opened);
        
        

        imgui::Window::new("Select Monster").position([0., 0.], imgui::Condition::Appearing).always_auto_resize(true).build(ui, || {
            imgui::InputInt::new(ui, "Monster ID",&mut state.id).step(1).step_fast(10).build();
            if state.id as usize > state.creatures.len()-1{
                state.id = (state.creatures.len()-1) as i32;
            }
            ui.text(&state.creatures[state.id as usize].0);
        });
        
        
        
        
        let monster = &state.creatures[state.id as usize];
        monster.1.print(ui, &state);
        
        
        
        
        token.pop();

    });
}


