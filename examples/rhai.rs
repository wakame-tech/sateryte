use std::collections::HashMap;

use rhai::{Engine, EvalAltResult, Scope, AST};

#[derive(Debug, Clone)]
struct Player {
    pub name: String,
    pub max_hp: i64,
    pub hp: i64,
}

impl Player {
    fn new(name: String, max_hp: i64) -> Self {
        Self {
            name,
            max_hp,
            hp: max_hp,
        }
    }

    // NOTE: require `mut`
    // <https://rhai.rs/book/rust/getters-setters.html>
    fn get_name(&mut self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_hp(&mut self) -> i64 {
        self.hp
    }

    fn set_hp(&mut self, hp: i64) {
        self.hp = hp;
    }

    fn get_max_hp(&mut self) -> i64 {
        self.max_hp
    }

    fn set_max_hp(&mut self, max_hp: i64) {
        self.max_hp = max_hp;
    }
}

fn init_engine() -> Engine {
    let mut engine = Engine::new();
    engine
        .register_type::<Player>()
        .register_get_set("name", Player::get_name, Player::set_name)
        .register_get_set("hp", Player::get_hp, Player::set_hp)
        .register_get_set("max_hp", Player::get_max_hp, Player::set_max_hp)
        .register_fn("new_player", Player::new);
    engine
}

fn load_skills(engine: &mut Engine) -> Result<HashMap<String, AST>, Box<EvalAltResult>> {
    let skills = HashMap::from_iter(vec![(
        "skill1".into(),
        engine.compile_file("skills/skill1.rhai".into())?,
    )]);
    Ok(skills)
}

fn exec_skill(
    engine: &mut Engine,
    skills: &HashMap<String, AST>,
    player: Player,
) -> Result<Player, Box<EvalAltResult>> {
    let mut scope = Scope::new();
    let player = engine.call_fn(&mut scope, &skills["skill1"], "effect", (player,))?;
    Ok(player)
}

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = init_engine();
    let skills = load_skills(&mut engine)?;

    let player = Player::new("hoge".to_string(), 50);
    dbg!(&player);
    let player = exec_skill(&mut engine, &skills, player)?;
    dbg!(&player);
    Ok(())
}
