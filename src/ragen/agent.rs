use crate::ragen::base::Feature;

pub struct Agent {
    feature: Feature,
    statisfaction: i8,
    constraints: Vec<Constraint>,
    components: Vec<Agent>,
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
            feature: Feature::new(),
            statisfaction: 0,
            constraints: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn components(&self) -> &Vec<Agent> {
        &self.components
    }

    pub fn get_statisfaction(&self) -> i8 {
        self.statisfaction
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
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
