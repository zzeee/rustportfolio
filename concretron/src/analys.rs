use futures::StreamExt;
use std::thread;
use std::time::Duration;
const maxdelay:usize=25;
pub const maxsize:usize=51;
pub const aos:usize=25;
pub const arr_size:usize=512;
use std::sync::mpsc;


pub type calculationLine=[f64;arr_size];
/*
impl std::fmt::Display for calculationLine {
fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(value a: {})", self)
}
}*/

// http://paulbourke.net/miscellaneous/correlate/
pub fn calcOne(x:&calculationLine,y:&calculationLine,smx:f64,smy:f64,delay:i32)->f64 {
    let mut atop:f64=0.;
    let mut btm1:f64=0.;
    let mut btm2:f64=0.;
    for i in 0..arr_size {
        let dd = (i as i32) + delay;
        if dd < 0 || dd >= arr_size as i32 { continue; }
        let mpointer = dd as usize;
        atop = atop + ((x[i] - smx) * (x[mpointer] - smy));
        btm1 = btm1 + (x[i] - smx) * (x[i] - smx);
        btm2 = btm2 + (y[mpointer] - smy) * (y[mpointer] - smy);
    }
    let cvalue:f64=atop/(btm1.sqrt()*btm2.sqrt());
    cvalue
}

pub fn cc(x:&calculationLine,y:&calculationLine)->[f64;maxsize] {
    const size:usize=arr_size;
    let mut res:[f64;maxsize]=[0.;maxsize];
    let mut mx:f64=0.;
    let mut my:f64=0.;
    let mut sx:f64;
    let mut sy:f64;
    let sxy:f64;
    for n in 0..size {
        mx += x[n];
        my += y[n];
    }
    mx = mx/(size as f64);
    my = my/(size as f64);
    sx = 0.;
    sy = 0.;
    let smx=mx.clone();
    let smy=my.clone();
    for n in 0..size {
        sx = sx+(x[n] -  smx) * (x[n] - smx);
        sy = sy+(y[n] - smy) * (y[n] - smy);
    }
    sxy = 0.;

    let mut delay:i32=-(maxdelay as i32);
    while delay <=maxdelay as i32 {
        let cvalue:f64;
        cvalue=calcOne(&x,&y,smx,smy,delay);
        let tval:usize=(delay +(aos as i32)) as usize;
        res[tval]=cvalue;
        delay=delay+1;
    }
    res

}
pub fn countMore(value:f64,ares:[f64;maxsize] )->u32 {
    let mut counter:u32=0;
    for (index,x) in ares.iter().enumerate() {
        if x>&value {counter=counter+1;}

    }
    counter
}



pub fn f64ArrStr( ind: &[f32])->String{
   // let as_string = format!("{:#}", ind);
   // println!("{}", as_string);
    for x in ind.iter() {

    }

    "SSSS".to_string()
}
pub fn findMaxInd(ares:[f64;maxsize])->usize {
    let mut mmax:f64=0.;
    let mut mind:usize=0;
    for (index,x) in ares.iter().enumerate() {
       // println!("XX {:} {:} {:}",x,index, mmax);
        if x>&mmax {
            //println!("dd");
            mmax=*x;
            mind=index;
        }
    }
    mind
}

//doPrint(mval)