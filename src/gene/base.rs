use crate::ragen::agent::{Constraint, ConstraintStruct};



// A constraint for surfacic agents to have a minimum size.
pub struct SizeConstraint {
    pub constraint_data: ConstraintStruct,
}


impl Constraint for SizeConstraint {
    fn get_agent(&self) -> &crate::ragen::agent::Agent { &self.constraint_data.agent }
    fn get_importance(&self) -> f64 { self.constraint_data.importance }
    fn get_priority(&self) -> i8 { self.constraint_data.priority }
    fn get_satisfaction(&self) -> f64 { self.constraint_data.statisfaction }
    fn set_satisfaction(&mut self, satisfaction: f64) { self.constraint_data.statisfaction = satisfaction; }
    fn is_hard(&self) -> bool { false }

    fn compute_initial_value(&self) {
        todo!()
    }

    fn compute_current_value(&self) {
        todo!()
    }

    fn compute_goal_value(&self) {
        todo!()
    }

    fn compute_satisfaction(&self) {
        todo!()
    }

    fn get_transformations(&self) -> Vec<Box<dyn crate::ragen::agent::Transformation>> {
        todo!()
    }

}

