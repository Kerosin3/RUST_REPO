use derivative::Derivative;
fn main() {
    //let mut player1 = PlayerObj::default();
    let mut player1 = PlayerObj::new(Weapon::Arms, Mood::Scary);
    let player1_speed = player1.adjust_speed();
    println!("player run speed: {player1_speed}");
    let player_speed_affected_mood =
        CalcSpeedPlayerAffectedMoodAndWeapon::new(Box::new(player1)).adjust_speed();
    println!(
        "player run speed affected mood: {player_speed_affected_mood}"
    );
}
#[derive(Derivative, Debug, Default)]
pub struct PlayerObj {
    #[derivative(Default(value = "Mood::Ok"))]
    mood: Mood,
    #[derivative(Default(value = "1.0_f32"))] //???
    run_speed_factor: f32,
    #[derivative(Default(value = "Weapon::Sword"))]
    weapon: Weapon,
}

impl PlayerObj {
    fn new(weapon: Weapon, mood: Mood) -> Self {
        Self {
            run_speed_factor: 1.0,
            weapon,
            mood,
        }
    }
}

pub trait Player {
    fn adjust_speed(&mut self) -> f32;
    fn get_mood(&self) -> Mood;
}

impl Player for PlayerObj {
    fn adjust_speed(&mut self) -> f32 {
        match self.weapon {
            Weapon::Sword => self.run_speed_factor *= 0.8,
            Weapon::Knife => self.run_speed_factor *= 0.9,
            Weapon::Arms => self.run_speed_factor *= 0.95,
        }
        self.run_speed_factor
    }
    fn get_mood(&self) -> Mood {
        self.mood
    }
}

pub struct CalcSpeedPlayerAffectedMoodAndWeapon {
    player: Box<dyn Player>,
}

impl CalcSpeedPlayerAffectedMoodAndWeapon {
    fn new(a_player: Box<dyn Player>) -> Self {
        Self { player: a_player }
    }
}

impl Player for CalcSpeedPlayerAffectedMoodAndWeapon {
    fn adjust_speed(&mut self) -> f32 {
        let speed_weapon = self.player.adjust_speed();
        match self.get_mood() {
            Mood::Ok => speed_weapon * 1.0,
            Mood::Creepy => speed_weapon * 0.8,
            Mood::Cheerful => speed_weapon * 1.2,
            Mood::Scary => speed_weapon * 1.5,
        }
    }
    fn get_mood(&self) -> Mood {
        self.player.get_mood()
    }
}
#[derive(Default, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Mood {
    #[default]
    Ok,
    Creepy,
    Cheerful,
    Scary,
}
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub enum Weapon {
    Sword,
    Knife,
    #[default]
    Arms,
}
