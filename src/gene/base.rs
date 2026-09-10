use geos::Geom;

use crate::ragen::agent::{Agent, Constraint, ConstraintStruct};



// A constraint for surfacic agents to have a minimum size.
pub struct SizeConstraint {
    pub constraint_data: ConstraintStruct,

    min_size:f64,
    size_deletion:f64,

    initial_area:f64,
    current_area:f64,
    goal_area:f64,

}

impl SizeConstraint {

    pub fn new(agent:Agent, min_size:f64, size_deletion:f64) -> Self {
        SizeConstraint {
            constraint_data: ConstraintStruct {
                agent: agent,
                statisfaction: 0.0,
                importance: 1.0,
                priority: 0,
                hard: false,
            },
            min_size: min_size,
            size_deletion: size_deletion,
            initial_area: -1.0,
            current_area: -1.0,
            goal_area: -1.0,
        }
    }
}


impl Constraint for SizeConstraint {

    fn get_agent(&self) -> &crate::ragen::agent::Agent { &self.constraint_data.agent }
    fn get_importance(&self) -> f64 { self.constraint_data.importance }
    fn get_priority(&self) -> i8 { self.constraint_data.priority }
    fn get_satisfaction(&self) -> f64 { self.constraint_data.statisfaction }
    fn set_satisfaction(&mut self, satisfaction: f64) { self.constraint_data.statisfaction = satisfaction; }
    fn is_hard(&self) -> bool { false }

    fn compute_initial_value(&mut self) {
        self.compute_current_value();
        self.initial_area = self.current_area;
    }

    fn compute_current_value(&mut self) {
        match self.constraint_data.agent.feature().geometry.area() {
            Ok(area) => self.current_area = area,
            Err(e) => println!("Error: {}", e),
        }
    }

    fn compute_goal_value(&mut self) {
        if self.initial_area <= self.size_deletion {
            self.goal_area = 0.0;
        } else if self.initial_area <= self.min_size {
            self.goal_area = self.min_size;
        } else {
            self.goal_area = self.initial_area;
        }
    }

    fn compute_satisfaction(&self) {
        if self.constraint_data.agent.is_deleted() {
            self.constraint_data.statisfaction = if self.goal_area == 0.0 { 10.0 } else { 0.0 };
            return;
        }
        if self.goal_area == 0.0 {
            self.constraint_data.statisfaction = 0.0;
            return;
        }
		self.constraint_data.statisfaction = 10.0 - 10.0 * (self.goal_area - self.current_area).abs() / self.goal_area;
		if(self.constraint_data.statisfaction < 0.0) { self.constraint_data.statisfaction = 0.0; }
    }

    fn get_transformations(&self) -> Vec<Box<dyn crate::ragen::agent::Transformation>> {
        todo!()
    }

}

