use std::cmp::Reverse;

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
pub static SATISFACTION_RESOLUTION: f64 = 0.00001;


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

	pub fn is_satisfied(&self) -> bool {
        10.0 - self.get_satisfaction() < SATISFACTION_RESOLUTION
    }


    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }

    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }

    // by default, the average of the satisfactions of the soft constraints. 0 if any hard constraint is unsatisfied.
    pub fn compute_satisfaction(&mut self) {
        if self.constraints.is_empty() || self.deleted {
            self.satisfaction = 10.0;
            return;
        }
        let mut t_sat: f64 = 0.0;
        let mut t_imp: f64 = 0.0;
        for c in &self.constraints {
            let imp = c.get_importance();
            if imp <= 0.0 { continue; }

            c.compute_current_value();
            c.compute_goal_value(); //TODO may not be necessary ? The goal value computation may be needed only once, at constraint creation

            // compute constraint satisfaction
            c.compute_satisfaction();
            let sat = c.get_satisfaction();
			if sat < 0.0 {
				eprintln!("Constraint with negative satisfaction found: {}", c.get_message());
			} else if sat > 10.0 {
				eprintln!("Constraint with satisfaction above 10 found: {}", c.get_message());
			}

            // case constraint is hard
			if c.is_hard() && sat < 10.0 {
				self.satisfaction = 0.0;
				return;
			}
			if c.is_hard() { continue; }

            t_sat += sat * imp;
            t_imp += imp;
        }
        if t_imp == 0.0 { self.satisfaction = 10.0; } else { self.satisfaction = t_sat / t_imp; }
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


	// retrieve list of candidate transformations to try improving agent's satisfaction
    fn get_transformations(&mut self) -> Vec<Box<dyn Transformation>> {
        let mut tr: Vec<Box<dyn Transformation>> = Vec::new();
        if self.deleted { return tr; }

        //sort constraints by priority
        //TODO need to do that only once ? Or when a constraint is added ?
        self.constraints.sort_by_key(|c| Reverse(c.get_priority()));

        for c in &self.constraints {
            if c.get_satisfaction() == 10.0 { continue; }
            let mut c_tr = c.get_transformations();
            tr.append(&mut c_tr);
        }
        tr
    }

    // activate agent: try to improve its satisfaction by applying transformations proposed by its constraints.
    pub fn activate(&mut self) {

        if self.is_frozen() || self.deleted { return; }

        //compute satisfaction
        self.compute_satisfaction();

		//satisfaction perfect: nothing to do.
        if self.is_satisfied() { return; }

        // store current satisfaction
		let mut sat1 = self.get_satisfaction();

		//get list of candidate transformations from agent
        let mut ts = self.get_transformations();

        while ts.len() > 0 {
            // pop first transformation from list
            let t = ts.remove(0);

            //save current state
            if t.is_cancellable() { t.store_state(); }

			//apply transformation
            t.apply();

			//TODO check proposing constraint satisfaction improvement first. Propose generic validity function?

            //get new satisfaction
			self.compute_satisfaction();
			let sat2 = self.get_satisfaction();

			if self.is_satisfied() {
				//perfect state reached: end
				return
			} else if sat2 - sat1 > SATISFACTION_RESOLUTION {
				//improvement: get new list of candidate transformations
				ts = self.get_transformations();
				sat1 = sat2;
			} else {
				//no improvement: go back to previous state, if possible
				if t.is_cancellable() {
					t.cancel();
                }
				else if sat2 - sat1 < 0.0 {
					//("Non cancellable transformation "+t.getClass().getSimpleName()+" resulted in satisfaction decrease for agent "+this.getId() + "   SatIni="+sat1+" --- satFin="+sat2+" --- diff="+(sat2-sat1));
                }
			}

        }
    }

}


//#[derive(Debug)]
pub struct ConstraintStruct {
    pub agent: Agent,
    pub statisfaction: f64,
    pub importance: f64,
    pub priority: i8,
    pub hard: bool,
}

pub trait Constraint {
    fn get_agent(&self) -> &Agent;

    // importance (used for soft constraints only, to compute agent's overall satisfaction).
    fn get_importance(&self) -> f64;

    fn get_priority(&self) -> i8;

	// A constraint whose satisfaction is expected to be 0 or 10, which has to be satisfied. Example: a topological constraint.
    //TODO a hard constraint does not need importance value ?
    fn is_hard(&self) -> bool;

    //TODO ensures it is used on constraint creation
    fn compute_initial_value(&self);
    fn compute_current_value(&self);
    fn compute_goal_value(&self);
    fn compute_satisfaction(&self);
    fn get_transformations(&self) -> Vec<Box<dyn Transformation>>;

    //from 0 to 10 (satisfied)
    fn get_satisfaction(&self) -> f64;
    fn set_satisfaction(&mut self, satisfaction: f64);
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
	fn compute_satisfaction(&mut self) {
        let satisfaction = if self.is_applied() { 10.0 } else { 0.0 };
        self.set_satisfaction(satisfaction);
    }

    fn get_transformations(&self) -> Vec<Box<dyn Transformation>> {
        let tr: Vec<Box<dyn Transformation>> = vec![self.get_transformation()];
        self.set_applied(true);
        tr
    }

}




pub trait Transformation {
    fn get_agent(&self) -> &Agent;
    fn apply(&self);
 
    fn is_cancellable(&self) -> bool;
	fn store_state(&self);	
	fn cancel(&self);	

    fn to_string(self) -> String;
}

// A transformation, which cannot be cancelled.
// In theory, all transformations could be cancellable, as soon as the initial state can be stored. In practice, it is not always easy and implemented.
//pub trait TransformationNoncancellable : Transformation {}


//A transformation, which can be cancelled.
//pub trait TransformationCancellable<> : Transformation {

