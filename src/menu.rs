use bevy::{app::AppExit, prelude::*};

use crate::{schedule::GameState, IsFirstRun};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(FixedUpdate, menu_action.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), teardown_menu);
    }
}

const TEXT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;
const BUTTON_COLOR: Color = Color::DARK_GRAY;
const BORDER_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct Disabled;
#[derive(Component)]
struct MenuItem;

#[derive(Component)]
enum MenuButtonAction {
    Resume,
    New,
    Quit,
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PixelifySans-VariableFont_wght.ttf");

    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };
    commands
        .spawn((
            NodeBundle {
                style: {
                    Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    }
                },
                ..default()
            },
            MenuItem,
        ))
        // Vertical Column Container
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    border_color: BORDER_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Game Title
                    parent.spawn(
                        TextBundle::from_section(
                            "Bevy Pong",
                            TextStyle {
                                font,
                                font_size: 80.,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::Resume,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Resume",
                                button_text_style.clone(),
                            ));
                        });
                    // New Game Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::New,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                button_text_style.clone(),
                            ));
                        });
                    // Exit Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Exit", button_text_style.clone()));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    is_first_run: Res<IsFirstRun>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Resume if !**is_first_run => game_state.set(GameState::Playing),
                MenuButtonAction::New => game_state.set(GameState::Reset),
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                _ => {}
            }
        }
    }
}

fn teardown_menu(mut commands: Commands, despawn_query: Query<Entity, With<MenuItem>>) {
    for entity in &despawn_query {
        commands.entity(entity).despawn_recursive();
    }
}
