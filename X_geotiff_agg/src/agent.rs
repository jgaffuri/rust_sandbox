

#[derive(Debug)]
struct Agent {
    statisfaction: i16,
}

impl Agent {

    fn Agent(satisfaction: i16) -> Self {
        Self {
            satisfaction: satisfaction
        }
    }

    fn compute_satisfaction(& mut self) -> () {
        self.statisfaction = 1000
    }
}


//fn main() {}


