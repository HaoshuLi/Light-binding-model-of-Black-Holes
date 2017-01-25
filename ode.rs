fn one_step(fs : &Vec<fn(f64, &Vec<f64>)->f64>, t : f64, ws : &Vec<f64>, h : f64)
            -> (f64,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>) {
    let len = ws.len();
    let s1 = fs.iter().map(|f| f(t,ws)).collect::<Vec<f64>>();
    let mut arg : Vec<f64> = Vec::new();
    for i in 0..len {
        arg.push(ws[i]+h*s1[i]/4.0);
    }
    let s2 = fs.iter().map(|f| f(t+h/4.0, &arg)).collect::<Vec<f64>>();
    arg.clear();
    for i in 0..len {
        arg.push(ws[i]+3.0*h*s1[i]/32.0+9.0*h*s2[i]/32.0);
    }
    let s3 = fs.iter().map(|f| f(t+3.0*h/8.0, &arg)).collect::<Vec<f64>>();
    arg.clear();
    for i in 0..len {
        arg.push(ws[i]+1932.0*h*s1[i]/2197.0-7200.0*h*s2[i]/2197.0+7296.0*h*s3[i]/2197.0);
    }
    let s4 = fs.iter().map(|f| f(t+12.0*h/13.0, &arg)).collect::<Vec<f64>>();
    arg.clear();
    for i in 0..len {
        arg.push(ws[i]+439.0*h*s1[i]/216.0-8.0*h*s2[i]+3680.0*h*s3[i]/513.0-845.0*h*s4[i]/4104.0);
    }
    let s5 = fs.iter().map(|f| f(t+h, &arg)).collect::<Vec<f64>>();
    arg.clear();
    for i in 0..len {
        arg.push(ws[i]-8.0*h*s1[i]/27.0+2.0*h*s2[i]-3544.0*h*s3[i]/2565.0+1859.0*h*s4[i]/4104.0-11.0*h*s5[i]/40.0);
    }
    let s6 = fs.iter().map(|f| f(t+h/2.0, &arg)).collect::<Vec<f64>>();

    let mut e : f64 = 0.0;
    for i in 0..len {
        let x = (s1[i]/360.0-128.0*s3[i]/4275.0-2197.0*s4[i]/75240.0+s5[i]/50.0+2.0*s6[i]/55.0).abs()*h;
        if x > e {
            e = x;
        }
    }

    (e, s1, s2, s3, s4, s5, s6)
}

fn one_succeed_step(fs : &Vec<fn(f64, &Vec<f64>)->f64>, t : f64, ws : &Vec<f64>, h0 : f64, norm : &mut f64, tof : f64)
                    -> (f64,Vec<f64>) {
    let mut h = h0;
    let (e,s1,_,s3,s4,s5,s6) = one_step(&fs, t, ws, h0);
    let t_s1 : Vec<f64>;
    let t_s3 : Vec<f64>;
    let t_s4 : Vec<f64>;
    let t_s5 : Vec<f64>;
    let t_s6 : Vec<f64>;
    if e/(*norm) >=tof {
        h = 0.8*((tof*(*norm)/e).powf(0.2))*h0;
        let (e,s1,_,s3,s4,s5,s6) = one_step(&fs, t, ws, h);
        if e/(*norm) >=tof {
            h = h/2.0;
            loop {
                let (e,s1,_,s3,s4,s5,s6) = one_step(&fs, t, ws, h);
                if e/(*norm) >= tof {
                    h=h/2.0;
                }
                else {
                    t_s1=s1; t_s3=s3; t_s4=s4; t_s5=s5; t_s6=s6;
                    break;
                }
            }
        }
        else {
            t_s1=s1; t_s3=s3; t_s4=s4; t_s5=s5; t_s6=s6;
        }
    }
    else {
        t_s1=s1; t_s3=s3; t_s4=s4; t_s5=s5; t_s6=s6;
    }
    let mut result : Vec<f64> = Vec::new();
    *norm = 0.0;
    for i in 0..ws.len() {
        let x = ws[i]+h*(16.0*t_s1[i]/135.0+6656.0*t_s3[i]/12825.0+28561.0*t_s4[i]/56430.0-9.0*t_s5[i]/50.0+2.0*t_s6[i]/55.0);
        if x.abs() > *norm {
            *norm=x.abs();
        }
        result.push(x);
    }
    (t+h,result)
}

pub fn solve(fs : &Vec<fn(f64, &Vec<f64>)->f64>, ta : f64, term_cond : fn(&Vec<f64>)->bool,
             fail_cond : fn(&Vec<f64>)->bool,
         ws0 : &Vec<f64>, h0 : f64, tof : f64) -> (bool,Vec<(f64,Vec<f64>)>) {
    let mut result : Vec<(f64,Vec<f64>)> = Vec::new();
    result.push((ta,ws0.clone()));
    let mut norm=0.0;
    for w in ws0 {
        if w.abs()>norm {
            norm=w.abs();
        }
    }
    
    loop {
        if term_cond(&result[result.len()-1].1) {
            return (true,result);
        }
        else if fail_cond(&result[result.len()-1].1) {
            return (false,result);
        }
        else {
            let t=result[result.len()-1].0;
            let ws=(result[result.len()-1].1).clone();
            result.push(one_succeed_step(&fs,t,&ws,h0,&mut norm,tof));
        }
    }
}

pub fn solve_final_result(fs : &Vec<fn(f64, &Vec<f64>)->f64>, ta : f64, term_cond : fn(&Vec<f64>)->bool,
             fail_cond : fn(&Vec<f64>)->bool,
         ws0 : &Vec<f64>, h0 : f64, tof : f64) -> Option<Vec<f64>> {
    let mut result : Vec<f64> = ws0.clone();
    let mut norm=0.0;
    let mut cur_t = ta;
    for w in ws0 {
        if w.abs()>norm {
            norm=w.abs();
        }
    }
    
    loop {
        if term_cond(&result) {
            return Some(result)
        }
        else if fail_cond(&result) {
            return None
        }
        else {
            let (t, ws) = one_succeed_step(&fs,cur_t,&result,h0,&mut norm,tof);
            cur_t = t;
            result = ws;
        }
    }
}


            
    
