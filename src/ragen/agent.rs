use crate::ragen::base::Feature;

//See https://github.com/eurostat/JGiscoTools/tree/master/modules/agent/src/main/java/eu/europa/ec/eurostat/jgiscotools/agent

pub struct Agent {
    id: u32,
    feature: Feature,
    satisfaction: i8,
    constraints: Vec<Box<dyn ConstraintTrait>>,
    components: Vec<Agent>,
    deleted: bool,
    frozen: bool,
}
static mut AGENT_ID: u32 = 0;


impl Agent {
    pub fn new(feature:Feature) -> Self {
        Agent {
            id: unsafe {
                AGENT_ID += 1;
                AGENT_ID
            },
            feature: feature,
            satisfaction: 0,
            constraints: Vec::new(),
            components: Vec::new(),
            deleted: false,
            frozen: false,
        }
    }

    pub fn get_id(&self) -> u32 {
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

    pub fn get_satisfaction(&self) -> i8 {
        self.satisfaction
    }

    pub fn add_constraint(&mut self, constraint: Box<dyn ConstraintTrait>) {
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
        let mut total_satisfaction: i16 = 0;
        let mut total_importance: i16 = 0;
        for c in &self.constraints {
            c.compute_current_value();
            c.compute_goal_value();
            c.compute_satisfaction();

			if(c.get_satisfaction()<0) {
				eprintln!("Constraint with negative satisfaction found: {}", c.get_message());
			} else if c.get_satisfaction() > 10 {
				eprintln!("Constraint with satisfaction above 10 found: {}", c.get_message());
			}

			if(c.is_hard() && c.get_satisfaction()<10) {
				self.satisfaction = 0;
				return;
			}
			if c.is_hard() { continue; }
            total_satisfaction += (c.get_satisfaction() as i16) * (c.get_importance() as i16);
            total_importance += c.get_importance() as i16;
        }
        self.satisfaction = (total_satisfaction/total_importance) as i8;
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
pub struct Constraint {
    agent: Agent,
    statisfaction: i8,
    importance: i8,
    priority: i8,
    hard: bool,
}

pub trait ConstraintTrait {
    fn get_agent(&self) -> &Agent;

    fn get_importance(&self) -> i8;
    fn get_priority(&self) -> i8;
    fn is_hard(&self) -> bool;

    fn compute_initial_value(&self);
    fn compute_current_value(&self);
    fn compute_goal_value(&self);
    fn compute_satisfaction(&self);
    fn get_transformations(&self) -> Vec<Box<dyn Transformation>>;

    fn get_satisfaction(&self) -> i8;
    fn is_satisfied(&self, satisfaction_resolution: f64) -> bool {
        ((10 - self.get_satisfaction()) as f64) < satisfaction_resolution
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

impl Constraint {
    pub fn new(agent: Agent, statisfaction: i8, importance: i8, priority: i8, hard: bool) -> Self {
        Constraint {
            agent,
            statisfaction,
            importance,
            priority,
            hard,
        }
    }

    pub fn get_agent(&self) -> &Agent {
        &self.agent
    }

    pub fn get_statisfaction(&self) -> i8 {
        self.statisfaction
    }

    pub fn get_importance(&self) -> i8 {
        self.importance
    }

    pub fn get_priority(&self) -> i8 {
        self.priority
    }

    pub fn is_hard(&self) -> bool {
        self.hard
    }

}


/**
 * A constraint to force a transformation to be applied.
 * The moment when the transformation is to be applied can be adjusted with the constraint priority.
 */
pub trait ConstraintOneShot : ConstraintTrait {
    fn get_transformation(&self) -> Box<dyn Transformation>;
    fn compute_current_value(&self) {}
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


pub struct SizeConstraint {
    pub constraint: Constraint,
}
