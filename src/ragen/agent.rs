pub struct Agent {
    pub statisfaction: i8,
    pub constraints: [Constraint; 1],
}

//#[derive(Debug)]
pub struct Constraint {
    pub statisfaction: i8,
}
