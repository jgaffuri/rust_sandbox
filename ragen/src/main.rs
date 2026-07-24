//#[derive(Debug)]
struct Agent {
    statisfaction: i8,
    constraints: [Constraint;1]
}

//#[derive(Debug)]
struct Constraint {
    statisfaction:i8
}


fn main() {
    //println!("Hello, world!");


    let c1 = Constraint { statisfaction: 5 };
    let ag1 = Agent { statisfaction: 5, constraints: [c1] };

    //dbg!(&ag1);
    println!("{}", &ag1.statisfaction);
    println!("{}", &ag1.constraints[0].statisfaction);
}
