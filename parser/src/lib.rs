
mod desc_parser;
pub use desc_parser::{UiInstruction,parse};

mod state;
use state::State;


#[warn(unused)]
fn main() {
    //let string = "<p data-visibility=\"gm\"><strong>Frequency</strong> once per day;</p>\n<p><strong>Effect</strong> The irnakurse unleashes an alien shriek of nightmarish horror and pain. All non-evil creatures within a 10-foot emanation must attempt a <span data-pf2-check=\"will\" data-pf2-dc=\"28\" data-pf2-traits=\"auditory,concentrate,emotion,enchantment,mental,occult\" data-pf2-label=\"Soul Scream DC\" data-pf2-show-dc=\"gm\">Will</span> save.</p><p data-visbility=\"gm\">The irnakurse can Sustain Soul Scream for up to 6 rounds; each time it does, it repeats the effect.</p>\n<hr />\n<p><strong>Critical Success</strong> The creature is unaffected, and is temporarily immune to Soul Scream for 24 hours.</p>\n<p><strong>Success</strong> The creature is @Compendium[pf2e.conditionitems.Stupefied]{Stupefied 1} for 1 round.</p>\n<p><strong>Failure</strong> The creature is stupefied 1. Further failed saves against Soul Scream increase the condition value by 1, to a maximum of stupefied 4. Each time the character gets a full night's rest, the stupefied condition gained from Soul Scream decreases by 1.</p>\n<p><strong>Critical Failure</strong> As failure, except the stupefied value increases by 2 instead of by 1.</p>".to_string();

    //let string = "<p>You utter an arcane word of power that can make the target @Compendium[pf2e.conditionitems.Blinded]{Blinded} upon hearing it. Once targeted, the target is then temporarily immune for 10 minutes. The effect of the spell depends on the target's level.</p>\n<hr />\n<p><strong>11th or Lower</strong> The target is Blinded permanently.</p>\n<p><strong>12th-13th</strong> The target is Blinded for [[/br 1d4 #minutes]]{1d4 minutes}.</p>\n<p><strong>14th or Higher</strong> The target is @Compendium[pf2e.conditionitems.Dazzled]{Dazzled} for 1 minute.</p>\n<hr />\n<p><strong>Heightened (+1)</strong> The levels at which each outcome applies increase by 2.</p>".to_string();

    let string = "<p>[[/r 2d8 #negative]]{2d8 negative damage} plus [[/r 1d6 #fire]]{1d6 fire damage}, <span data-pf2-check=\"fortitude\" data-pf2-traits=\"damaging-effect\" data-pf2-label=\"Constrict DC\" data-pf2-dc=\"26\" data-pf2-show-dc=\"gm\">basic Fortitude</span></p>\n<hr />\n<p>The monster deals the listed amount of damage to any number of creatures @Compendium[pf2e.conditionitems.Grabbed]{Grabbed} or @Compendium[pf2e.conditionitems.Restrained]{Restrained} by it. Each of those creatures can attempt a basic Fortitude save with the listed DC.</p>".to_string();

    let clean_str = parse(string, 6);   

    let mut state = State::new();

    state.instruction = clean_str.0;
    state.instruction_level = clean_str.1;

    let mut executor = render_api::Executor::default();

    let config: render_api::Config<State> = render_api::Config{
        window_title: "Rot's Campaing Manager".to_string(),
        initial_window_width: 1280.,
        initial_window_height: 720.,        
        font_size: Some(15.),
        ..Default::default()
    };



    executor.build(&config, &mut state, |mut uploader, state|{
        
        
        
        state.insert_font(&mut uploader, "fonts/Rubik-Light.ttf", 15., None, 0);
        state.insert_font(&mut uploader, "fonts/Rubik-Bold.ttf", 15., None, 1);     
        
       
       
        uploader.ctx().style_mut().frame_rounding = 12.;
        uploader.ctx().style_mut().window_rounding = 12.;
        uploader.ctx().style_mut().child_rounding = 12.;
        uploader.ctx().style_mut().popup_rounding = 12.;
        uploader.ctx().style_mut()[imgui::StyleColor::WindowBg] = [0.04, 0.04, 0.04 ,1.];
        uploader.ctx().style_mut()[imgui::StyleColor::PopupBg] = [0.1, 0.1, 0.1 ,1.];
        
    });

    executor.run(config, state, |ui, state| {

        let token = ui.push_font(state.font(0));
        
        imgui::Window::new("button").position([0., 0.], imgui::Condition::Appearing).always_auto_resize(true).build(ui, || {
            if ui.button("toggle"){
                state.with_level = !state.with_level;
            }
        });
        
        
        

        imgui::Window::new("Desc").position([640., 360.], imgui::Condition::Appearing).always_auto_resize(true).build(ui, || {
            match state.with_level{
                true => {
                    for inst in state.instruction.iter(){
                        inst.print(ui, state)
                    }
                },
                false => {
                    for inst in state.instruction_level.iter(){
                        inst.print(ui, state)
                    }
                },
            }
        });
        
        
        
        
        
        
        
        
        token.pop();

    });

}
