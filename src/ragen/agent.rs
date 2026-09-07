use crate::ragen::base::Feature;

//See https://github.com/eurostat/JGiscoTools/tree/master/modules/agent/src/main/java/eu/europa/ec/eurostat/jgiscotools/agent

pub struct Agent {
    id: u64,
    feature: Feature,
    satisfaction: f64,
    constraints: Vec<Box<dyn Constraint>>,
    components: Vec<Agent>,
    deleted: bool,
    frozen: bool,
}
static mut AGENT_ID: u64 = 0;


impl Agent {
    pub fn new(feature:Feature) -> Self {
        Agent {
            id: unsafe {
                AGENT_ID += 1;
                AGENT_ID
            },
            feature: feature,
            satisfaction: 0.0,
            constraints: Vec::new(),
            components: Vec::new(),
            deleted: false,
            frozen: false,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_agent_type(&self) -> String {
        //TODO
        std::any::type_name::<Self>().to_string()
    }

    pub fn components(&self) -> &Vec<Agent> {
        &self.components
    }

    pub fn feature(&self) -> &Feature {
        &self.feature
    }

    pub fn get_satisfaction(&self) -> f64 {
        self.satisfaction
    }

    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }

    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }

    pub fn compute_satisfaction(&mut self) {
        if self.constraints.is_empty() || self.deleted {
            self.satisfaction = 10;
            return;
        }
        let mut total_satisfaction: f64 = 0.0;
        let mut total_importance: f64 = 0.0;
        for c in &self.constraints {
            c.compute_current_value();
            c.compute_goal_value();
            c.compute_satisfaction();

			if c.get_satisfaction()<0 {
				eprintln!("Constraint with negative satisfaction found: {}", c.get_message());
			} else if c.get_satisfaction() > 10.0 {
				eprintln!("Constraint with satisfaction above 10 found: {}", c.get_message());
			}

			if c.is_hard() && c.get_satisfaction() < 10.0 {
				self.satisfaction = 0.0;
				return;
			}
			if c.is_hard() { continue; }
            total_satisfaction += c.get_satisfaction() * c.get_importance();
            total_importance += c.get_importance();
        }
        self.satisfaction = total_satisfaction/total_importance;
    }

    pub fn freeze(&mut self) {
        self.frozen = true;
    }
    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    pub fn activate(&mut self) {
        //TODO
    }

}


//#[derive(Debug)]
pub struct ConstraintStruct {
    pub agent: Agent,
    pub statisfaction: f64,
    pub importance: f64,
    pub priority: f64,
    pub hard: bool,
}

pub trait Constraint {
    fn get_agent(&self) -> &Agent;

    // importance (used for soft constraints only, to compute agent's overall satisfaction).
    fn get_importance(&self) -> f64;

    fn get_priority(&self) -> f64;

	// A constraint whose satisfaction is expected to be 0 or 10, which has to be satisfied. Example: a topological constraint.
    fn is_hard(&self) -> bool;

    fn compute_initial_value(&self);
    fn compute_current_value(&self);
    fn compute_goal_value(&self);
    fn compute_satisfaction(&self);
    fn get_transformations(&self) -> Vec<Box<dyn Transformation>>;

    //from 0 to 10 (satisfied)
    fn get_satisfaction(&self) -> f64;
    fn set_satisfaction(&self, satisfaction: f64);
    fn is_satisfied(&self, satisfaction_resolution: f64) -> bool {
        ((10.0 - self.get_satisfaction()) as f64) < satisfaction_resolution
    }

    fn get_message(&self) -> String {
        format!(
            "{},agentid={},type={},pri={},imp={},s={}",
            self.get_agent().get_agent_type(),
            self.get_agent().get_id(),
            std::any::type_name::<Self>(),
            self.get_priority(),
            self.get_importance(),
            self.get_satisfaction()
        )
    }

}


/**
 * A constraint to force a transformation to be applied.
 * The moment when the transformation is to be applied can be adjusted with the constraint priority.
 */
pub trait ConstraintOneShot : Constraint {
    fn get_transformation(&self) -> Box<dyn Transformation>;
    fn compute_current_value(&self) {}
    fn is_applied(&self) -> bool;
    fn set_applied(&self, applied: bool);
	fn compute_satisfaction(&self) {
        let satisfaction = if self.is_applied() { 10.0 } else { 0.0 };
        self.set_satisfaction(satisfaction);
    }

    fn get_transformations(&self) -> Vec<Box<dyn Transformation>> {
        let tr: Vec<Box<dyn Transformation>> = vec![self.get_transformation()];
        self.set_applied(true);
        tr
    }

}



//TODO use enum instead
pub trait Transformation {
    fn get_agent(&self) -> &Agent;
    fn apply(&self);
	fn is_cancelable(&self);
    fn to_string(self) -> String;
}

/** A transformation, which can be cancelled. */
pub trait TransformationCancellable<> : Transformation {
	fn is_cancelable(&self) { true; }
	fn store_state(&self);	
	fn cancel(&self);	
}

/** 
 * A transformation, which cannot be cancelled.
 * In theory, all transformations could be cancellable, as soon as the initial state can be stored. In practice, it is not always easy and implemented.
 */
pub trait TransformationNonCancellable<> : Transformation {
	fn is_cancelable(&self) { false; }
}

