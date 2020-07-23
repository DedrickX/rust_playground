#[derive(Debug)]
struct Stats {
    hp: u8,
    sp: u8,
}

#[derive(Debug)]
struct Monster {
    stats: Stats,
    friends: Vec<Friend>,
}

#[derive(Debug)]
struct Friend {
    loyality: u8,
}

impl Monster {
    fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.stats.heal(friend.loyality);
            println!("Healing for {}", friend.loyality);
        }
    }
}

impl Stats {
    fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}

pub fn test() {
    let mut monster = Monster {
        stats: Stats { hp: 10, sp: 10 },
        friends: vec![Friend { loyality: 2 }, Friend { loyality: 1 }],
    };
    monster.final_breath();
    println!("Monster after final breath: {:?}", monster);
}
