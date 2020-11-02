struct SeaCreature {
    noise: String,
}

impl SeaCreature {
    fn get_sound(&self) -> &str{
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", self.get_sound());
    }
}

fn main() {
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    creature.make_noise();
}
