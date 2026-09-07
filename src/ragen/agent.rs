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

//#[derive(Debug)]
pub struct Constraint {
    pub agent: Agent,
    pub statisfaction: i8,
    pub importance: i8,
    pub priority: i8,
    pub hard_constraint: bool,
}

//TODO check in jgiscotools
pub struct AgenState {
    feature: Feature,
    statisfaction: i8,
}

pub struct SizeConstraint {
    pub constraint: Constraint,
}

/*
pub trait compute_statisfaction {
    fn compute_statisfaction(&mut self);
}
 */

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

    pub fn run(&mut self) {
        //TODO
    }

}
