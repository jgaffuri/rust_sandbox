use crate::ragen::base::Feature;

//See https://github.com/eurostat/JGiscoTools/tree/master/modules/agent/src/main/java/eu/europa/ec/eurostat/jgiscotools/agent

pub struct Agent {
    id: u32,
    feature: Feature,
    statisfaction: i8,
    constraints: Vec<Constraint>,
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
            statisfaction: 0,
            constraints: Vec::new(),
            components: Vec::new(),
            deleted: false,
            frozen: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn components(&self) -> &Vec<Agent> {
        &self.components
    }

    pub fn feature(&self) -> &Feature {
        &self.feature
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
        if self.constraints.is_empty() || self.deleted {
            self.statisfaction = 10;
            return;
        }
        let mut total_statisfaction: i16 = 0;
        let mut total_importance: i16 = 0;
        for constraint in &self.constraints {
            total_statisfaction += (constraint.statisfaction as i16) * (constraint.importance as i16);
            total_importance += constraint.importance as i16;
        }
        self.statisfaction = (total_statisfaction/total_importance) as i8;
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


/*/TODO check in jgiscotools
pub struct AgenState {
    feature: Feature,
    statisfaction: i8,
}*/


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
    fn compute_current_value(&self) -> i8;

    fn compute_satisfaction(&self) -> String;
    fn get_satisfaction(&self) -> i8;
    fn is_satisfied(&self, satisfactionResolution: f64) -> bool {
        ((10 - self.get_satisfaction()) as f64) < satisfactionResolution
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



/*
pub trait compute_statisfaction {
    fn compute_statisfaction(&mut self);
}
 */


pub struct SizeConstraint {
    pub constraint: Constraint,
}
