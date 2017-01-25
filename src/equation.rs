mod ode;
use ode::solve;

const M : f64 = 1.0;
const E : f64 = 0.1;
const L : f64 = 1.0;
const THETA : f64 = 0.06656816378;//std::f64::consts::PI/10.0;
const R0 : f64 = 10.0;
const R1 : f64 = 5.0;

fn square(x : f64) -> f64 {
    x*x
}
fn cube(x : f64) -> f64 {
    x*x*x
}

fn f1(_ : f64, ys : &Vec<f64>) -> f64 {
    ys[1]
}
fn f2(_ : f64, ys : &Vec<f64>) -> f64 {
    -M*E*E/(1.0-2.0*M/ys[0])/square(ys[0])+M/(1.0-2.0*M/ys[0])/square(ys[0])*square(ys[1])+(1.0-2.0*M/ys[0])*square(L)/cube(ys[0])
}
fn f3(_ : f64, ys : &Vec<f64>) -> f64 {
    L/square(ys[0])
}


fn term(ys : &Vec<f64>) -> bool {
    (-ys[0] * ys[2].cos() >= R1)
}
fn fail(ys : &Vec<f64>) -> bool {
    (ys[0]<2.0*M) || (ys[0] * ys[2].cos() > R0+1.0) || ((ys[0] * ys[2].sin()).abs() > 6.0*R0)
}

fn main() {
    let fs : Vec<fn(f64,&Vec<f64>)->f64> = vec![f1 , f2, f3];
    let ws = vec![R0, -L/(THETA.tan())/R0, 0.0];
    let (flag,result) = solve(&fs, 0.0, term, fail, &ws, 0.5, 0.05);
    if flag {
        println!("succeed!");
    }
    else {
        println!("fall into the event horizon or fly away!");
    }
    for term in &result {
        println!("{} {}", (term.1)[0]*((term.1)[2]).cos(), (term.1)[0]*((term.1)[2]).sin());
    }
}
