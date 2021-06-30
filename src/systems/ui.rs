use bevy::prelude::*;

use crate::components::ui::*;
use crate::components::resources::*;

pub fn ui_system(
    mut _commands: Commands,
    _time: Res<Time>,
    gamestate: ResMut<GameState>,
    mut query: Query<(&UiTextElements, &mut Visible)>,
) {
    for (text_element, visible) in query.iter_mut() {
        match text_element {
            UiTextElements::RestartText => update_restart_text(&gamestate, visible)
        }
    }
}

fn update_restart_text(gamestate: &ResMut<GameState>, mut visible: Mut<Visible>){
    if matches!(gamestate.play_state, PlayState::Crashed) && !visible.is_visible {
        visible.is_visible = true;
    }
    if visible.is_visible && !matches!(gamestate.play_state, PlayState::Crashed) {
        visible.is_visible = false;
    }
}

pub trait UiExtensions {
    fn add_ui_elements(&mut self, asset_server: Res<AssetServer>);
}

impl UiExtensions for Commands<'_> {
    fn add_ui_elements(&mut self, asset_server: Res<AssetServer>) {
        self.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(5.0),
                        right: Val::Px(15.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "Press R To Restart",
                    TextStyle {
                        font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                ),
                visible: Visible {
                    is_visible: false,
                    ..Visible::default()
                },
                ..Default::default()
            })
            .insert(UiTextElements::RestartText);
    }
}