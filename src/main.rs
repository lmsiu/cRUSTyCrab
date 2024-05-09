// Source : https://github.com/bevyengine/bevy/blob/main/examples/games/game_menu.rs

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    GameOne,
    GameTwo,
    GameThree,
    GameFour,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        // Adds the plugins for each state
        .add_plugins(( 
            menu::menu_plugin, 
            gameone::gameone_plugin, 
            gametwo::gametwo_plugin,
            gamethree::gamethree_plugin,
            gamefour::gamefour_plugin,
        ))
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    commands.spawn(Camera2dBundle::default());
    game_state.set(GameState::Menu);
}

// work here maybe?
mod gameone {
    use bevy::prelude::*;
    use std::process::Command;

    use super::{despawn_screen, GameState};

    // GAME ONE PLUGIN
    pub fn gameone_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOne), gameone)
            .add_systems(OnExit(GameState::GameOne), despawn_screen::<OnGameScreen>);
    }

    // Tag component used to tag entities added on the game screen
    #[derive(Component)]
    struct OnGameScreen;

    fn gameone(mut game_state: ResMut<NextState<GameState>>) {
       Command::new("cargo").arg("run").arg("--bin").arg("gameone").output().expect("unable to run game one");
       game_state.set(GameState::Menu)
    }
}

mod gametwo {
    use bevy::prelude::*;
    use std::process::Command;

    use super::{despawn_screen, GameState};

    // GAME TWO PLUGIN
    pub fn gametwo_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::GameTwo), gametwo)
            .add_systems(OnExit(GameState::GameTwo), despawn_screen::<OnGameScreen>);
    }

    // Tag component used to tag entities added on the game screen
    #[derive(Component)]
    struct OnGameScreen;

    fn gametwo(mut game_state: ResMut<NextState<GameState>>) {
       Command::new("cargo").arg("run").arg("--bin").arg("gametwo").output().expect("unable to run game two");
       game_state.set(GameState::Menu);
    }
}

mod gamethree {
    use bevy::prelude::*;
    use std::process::Command;

    use super::{despawn_screen, GameState};

    // GAME THREE PLUGIN
    pub fn gamethree_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::GameThree), gameone)
            .add_systems(OnExit(GameState::GameThree), despawn_screen::<OnGameScreen>);
    }

    // Tag component used to tag entities added on the game screen
    #[derive(Component)]
    struct OnGameScreen;

    fn gameone(mut game_state: ResMut<NextState<GameState>>) {
       Command::new("cargo").arg("run").arg("--bin").arg("crabshooter").output().expect("unable to run game three");
       game_state.set(GameState::Menu)
    }
}

mod gamefour {
    use bevy::prelude::*;
    use std::process::Command;

    use super::{despawn_screen, GameState};

    // GAME FOUR PLUGIN
    pub fn gamefour_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::GameFour), gameone)
            .add_systems(OnExit(GameState::GameFour), despawn_screen::<OnGameScreen>);
    }

    // Tag component used to tag entities added on the game screen
    #[derive(Component)]
    struct OnGameScreen;

    fn gameone(mut game_state: ResMut<NextState<GameState>>) {
       Command::new("cargo").arg("run").arg("--bin").arg("autorunner").output().expect("unable to run game three");
       game_state.set(GameState::Menu)
    }
}

mod menu {
    use bevy::{app::AppExit, prelude::*};

    use super::{despawn_screen, GameState, TEXT_COLOR};

    // This plugin manages the menu
    pub fn menu_plugin(app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            );
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        #[default]
        Disabled,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    // Tag component used to mark which setting is currently selected
    #[derive(Component)]
    struct SelectedOption;

    // All actions that can be triggered from a button click
    // GAME BUTTONS
    #[derive(Component)]
    enum MenuButtonAction {
        PlayOne,
        PlayTwo,
        PlayThree,
        PlayFour,
        Quit,
    }

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Common style for all buttons on the screen
        let button_style = Style {
            width: Val::Px(275.0),
            height: Val::Px(90.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_style = Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            ..default()
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn(
                            TextBundle::from_section(
                                "CRUSTACEAN RECREATION",
                                TextStyle {
                                    font_size: 80.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            }),
                        );

                        // GAME BUTTONS
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::PlayOne,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Catch the Crab",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::PlayTwo,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Seafood Scramble",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::PlayThree,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Crab Shooter",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::PlayFour,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Crab Autorunner",
                                    button_text_style.clone(),
                                ));
                            });
                        
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/exitRight.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style,
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section("Quit", button_text_style));
                            });
                    });
            });
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit);
                    }
                    // ENTERS GAMES
                    MenuButtonAction::PlayOne => {
                        game_state.set(GameState::GameOne);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::PlayTwo => {
                        game_state.set(GameState::GameTwo);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::PlayThree => {
                        game_state.set(GameState::GameThree);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::PlayFour => {
                        game_state.set(GameState::GameFour);
                        menu_state.set(MenuState::Disabled);
                    }
                }
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}