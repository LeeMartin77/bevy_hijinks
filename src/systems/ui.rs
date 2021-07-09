use crate::components::entities::Player;
use crate::components::physical_attributes::Thrust;
use bevy::prelude::*;

use crate::components::resources::*;

pub enum UiTextElements {
    RestartText,
    ControlData
}


pub fn ui_system(
    mut _commands: Commands,
    _time: Res<Time>,
    gamestate: ResMut<GameState>,
    mut set: QuerySet<(
        Query<(&UiTextElements, &mut Visible, &mut Text)>,
        Query<(&Player, &Thrust)>
    )>
) {
    let mut player_thrust = Thrust { facing: 0.0, thrust: 0.0 };

    if let Ok((_player, thrust)) = set.q1().single() {
        player_thrust.facing = thrust.facing;
        player_thrust.thrust = thrust.thrust;
    }

    for (text_element, visible, text) in set.q0_mut().iter_mut() {
        match text_element {
            UiTextElements::RestartText => update_restart_text(&gamestate, visible),
            UiTextElements::ControlData => update_control_text(text, &player_thrust)
        }
    }
}

fn update_control_text(mut text: Mut<Text>, thrust: &Thrust){
    text.sections[0].value = THRUST_LABEL.to_string() + &thrust.thrust.to_string() + "\n";
    text.sections[1].value = FACING_LABEL.to_string() + &thrust.facing.to_string();
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

const THRUST_LABEL: &str = "Thrust: ";
const FACING_LABEL: &str = "Facing: ";

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

            self.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        left: Val::Px(15.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text{
                    sections: vec![
                        TextSection {
                            value: THRUST_LABEL.to_string() + "\n",
                            style: TextStyle {
                                font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            }
                        },
                        TextSection {
                            value: FACING_LABEL.to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            }
                        },
                    ],
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Left,
                        vertical: VerticalAlign::Bottom
                    },
                },
                visible: Visible::default(),
                ..Default::default()
            })
            .insert(UiTextElements::ControlData);
    }
}