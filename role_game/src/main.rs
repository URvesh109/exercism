#[allow(dead_code)]
#[derive(Debug)]
struct Player {
    health: u32,
    mana: Option<u32>,
    level: u32,
}

impl Player {
    fn revive(self) -> Option<Player> {
        match self.health {
            0 => {
                let p1 = Player {
                    health: 100,
                    mana: match self.level {
                        10.. => Some(100),
                        0..=9 => None,
                    },
                    ..self
                };
                Some(p1)
            }
            _ => None,
        }
    }

    fn cast_spell(&mut self, mana: u32) -> u32 {
        let mut value = 0;
        match self.mana {
            Some(v) if v > mana => {
                let damage = v - mana;
                self.mana = Some(damage);
                value = damage;
            }
            Some(_) => (),
            None => self.health -= mana,
        }
        value
    }
}

fn main() {
    let mut not_a_wizard_yet = Player {
        health: 79,
        mana: None,
        level: 9,
    };

    assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
    assert_eq!(not_a_wizard_yet.health, 74);
    assert_eq!(not_a_wizard_yet.mana, None);

    let mut low_mana_wizard = Player {
        health: 93,
        mana: Some(3),
        level: 12,
    };

    assert_eq!(low_mana_wizard.cast_spell(10), 0);
    assert_eq!(low_mana_wizard.health, 93);
    assert_eq!(low_mana_wizard.mana, Some(3));

    let mut wizard = Player {
        health: 123,
        mana: Some(30),
        level: 18,
    };
    assert_eq!(wizard.cast_spell(10), 20);
    assert_eq!(wizard.health, 123);
    assert_eq!(wizard.mana, Some(20));
}
