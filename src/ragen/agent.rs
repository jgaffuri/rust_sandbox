pub struct Agent {
    pub statisfaction: i8,
    pub constraints: [Constraint; 1],
}

//#[derive(Debug)]
pub struct Constraint {
    pub statisfaction: i8,
}

/*
pub trait compute_statisfaction {
    fn compute_statisfaction(&mut self);
}
 */

impl Agent {
    pub fn new() -> Self {
        Agent {
            statisfaction: 0,
            constraints: [Constraint { statisfaction: 0 }],
        }
    }

    pub fn compute_statisfaction(&mut self) {
        if self.constraints.is_empty() {
            self.statisfaction = 10;
            return;
        }
        let mut total_statisfaction: i16 = 0;
        for constraint in &self.constraints {
            total_statisfaction += constraint.statisfaction as i16;
        }
        total_statisfaction = total_statisfaction / self.constraints.len() as i16;
        self.statisfaction = total_statisfaction as i8;
    }
}
