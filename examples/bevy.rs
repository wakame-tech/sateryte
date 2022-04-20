use bevy::prelude::*;

#[derive(Debug, Component)]
struct Name(String);

/// 空のstructは marker として使える
#[derive(Component)]
struct Player;

/// [Bundle]: [Component] をまとめたもの
#[derive(Debug, Component)]
struct Status {
    hp: i32,
}

/// [Resource] は Default trait を実装する
#[derive(Debug, Component)]
struct SomeResource(String);

impl FromWorld for SomeResource {
    fn from_world(world: &mut World) -> Self {
        info!("{:?}", world);
        SomeResource(String::from("hello"))
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    _marker: Player,
    name: Name,
    status: Status,
}

impl PlayerBundle {
    fn new(name: &str, hp: i32) -> Self {
        Self {
            _marker: Player,
            name: Name(name.to_string()),
            status: Status { hp },
        }
    }
}

fn add_player(mut commands: Commands) {
    // Entity: ユニークなIDを持つ, Component の集合
    commands
        .spawn()
        .insert_bundle(PlayerBundle::new("Alice", 100));
}

/// [Query] を使って [Component] を取得して処理を行う
fn greet_player(_players: Query<(&Name, &Status), With<Player>>) {
    // for (name, status) in players.iter() {
    //     dbg!(name, status);
    // }
}

enum Action {
    Left,
    Right,
    Up,
    Down,
}

fn input_keys(
    mut char_input_events: EventReader<ReceivedCharacter>,
    mut sender: EventWriter<Action>,
) {
    for event in char_input_events.iter() {
        match event.char {
            'a' => sender.send(Action::Left),
            's' => sender.send(Action::Down),
            'd' => sender.send(Action::Right),
            'w' => sender.send(Action::Up),
            _ => {}
        }
    }
}

fn action_listener(mut reader: EventReader<Action>) {
    for action in reader.iter() {
        match action {
            Action::Left => info!("left"),
            Action::Right => info!("right"),
            Action::Up => info!("up"),
            Action::Down => info!("down"),
        }
    }
}

/// [Plugin] にして機能をまとめることが出来る
pub struct SomePlugin;

impl Plugin for SomePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Action>()
            .add_startup_system(add_player)
            .add_system(input_keys)
            .add_system(action_listener)
            .add_system(greet_player);
    }
}

fn init_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(640.0, 480.0);
    window.set_resizable(false);
}

/// [App] に [System] を登録する
/// `add_startup_system`: [App] の開始時に1度呼ばれる
/// `add_system`: 毎フレーム呼ばれる
///
/// [Resource]: App中のglobalでuniqなデータ
/// [Query]: [System] で使うための [Entity] を取得
/// [Commands]: [Entity] の生成, 破棄. [Component] の追加, 削除. [Resource] の管理
/// [Event]: [System] 同士のイベントやり取り
fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // or [MinimalPlugins]
        .init_resource::<SomeResource>() // resource init
        .add_startup_system(init_window)
        .add_plugin(SomePlugin)
        .run();
}
