use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use crate::console_log;
use crate::types::layersStorage::{CollisionsLine,  DocumentStorage, PaletteElements};
use crate::utils::main::log;
use ndarray::{Array, Array2, Array3, Axis, Shape};
use crate::blocks::blocks::AllMasks;
use crate::types::events::{CollisionZone};
use std::char;
use web_sys::console::assert;

pub fn getEls(source: u128, maxcount: u32, elementmaxcount: u32) -> (Vec<u128>, Vec<u32>, Vec<u32>) {
    let asource = source.clone();
    let mut msource = source;
    let mut els = vec![];
    let mut pwrs = vec![];
    let mut elements = vec![];
    for i in (1..=maxcount).rev() {
        if msource >> i == 1 {
            let el = 2_u128.pow(i);
            msource = msource - el;
            els.push(el);
            pwrs.push(i);
            // console_log!("eledel {:?}", i/elementmaxcount);
            elements.push(i / elementmaxcount);
        }
        // console_log!("print! {:?} {:?} {:?}",i, i/elementmaxcount,  msource )
    }
    elements.dedup();
    elements.sort();
    elements.dedup();

    // console_log!("source: , res:es {:?} {:?} {:?} {:?}",asource, els, pwrs, elements);
    (els, pwrs, elements)
}


pub fn printShape_u128(matrixarr:Array2<u128>,code: AllMasks, from:bool, comment:&str) {
     console_log!("printShape {:?} {:?} {:?}",matrixarr.shape(), matrixarr.row(0).shape(), matrixarr.column(0).shape());
    let width=matrixarr.shape()[1];
    let height=matrixarr.shape()[0];
 for arang in  0..height {
        // let line=matrixarr.index_axis(Axis(0),arang);
       // console_log!("alalalal {:?} {:?}", line.len(),arang);
         let line=matrixarr.row(arang);
        let line1=line.to_vec();
        let line2:Vec<&str>=line.to_vec().iter().map(|e| match *e {
            0 => " ",
            2 => "a",
            4 => "b",
            8 => "c",
            16 => "d",
            32 => "e",
            64 => "f",
            128 => "g",
            256 => "h",
            512 => "j",
            1024 => "k",
            _ =>"!"
        }).collect();
        console_log!("l2:{:?} {:?} {:?}",line2.join(""), line2.len(),comment);

    }

}
pub fn printShape32(matrixarr0:Array2<u32>, code: AllMasks, from:bool, comment:&str) {
    if !from { return ;}
   if code!=AllMasks::MaskInputs {return;}
    let matrixarr=matrixarr0.clone();
    let matrixarr2=matrixarr0.to_shape((matrixarr0.shape()[1],matrixarr0.shape()[0])).unwrap();

    console_log!("printShape {:?} {:?} {:?} {:?}",matrixarr.shape(),matrixarr2.shape(), matrixarr.shape()[0],matrixarr.shape()[1]);
    let width=matrixarr.shape()[0];
    let height=4 ; //matrixarr.shape()[1];

       // let mm=matrixarr.reshape();

            let line00=matrixarr.index_axis(Axis(0),0);
                let line01=matrixarr.index_axis(Axis(1),0);

    console_log!("aaprintShape32 {:?} {:?} {:?} ", line00.len(), line01.len(),comment);

//(,3)
    for arang in  0..height {
        console_log!("aaaaa {:?} {:?}", matrixarr.index_axis(Axis(1),0).len(),matrixarr.index_axis(Axis(0),0).len());
        // let line=matrixarr.index_axis(Axis(0),arang);
       // console_log!("alalalal {:?} {:?}", line.len(),arang);
         let line=matrixarr.row(arang);
        let mut line1=line.clone().to_vec();
        let mline1  =&line1[..];
        console_log!("line1 {:?} {:?} {:?}",comment,mline1,mline1.len());
        let line2:Vec<&str>=line.to_vec().iter().map(|e| match *e {
            0 => " ",
            1 => "a",
            2 => "b",
            4 => "c",
            8 => "d",
            16 => "e",
            32 => "f",
            64 => "g",
            128 => "h",
           // 256 => "h",
          //  512 => "j",
          //  1024 => "k",
            _ =>"!"
        }).collect();
        line1.sort();
        line1.dedup();
        console_log!("l2{:?} {:?} {:?} {:?} {:?} {:?}",line2.join(""), line2.len(), arang, line1, code,comment);
       // console_log!("A:{:?} {:?} {:?}",line2,code,comment);
    }
}
pub fn printShape(matrixarr:Array2<u8>, code: AllMasks, from:bool, comment:&str) {
    if !from { return ;}
   if code!=AllMasks::MaskInputs {return;}
    console_log!("printShape {:?} {:?} {:?} {:?} {:?}",matrixarr.shape(),matrixarr.shape()[0],matrixarr.shape()[1], matrixarr.row(0).shape(), matrixarr.column(0).shape());
    let width=matrixarr.shape()[1];
    let height= matrixarr.shape()[0];
         //   let line00=matrixarr.index_axis(Axis(1),0);
           // let line01=matrixarr.index_axis(Axis(0),0);
  //  console_log!("aa {:?} {:?} {:?} {:?} {:?}", line00.len(), line01.len(), width, height, comment);

//(,3)
    for arang in  0..height {
        //let line=matrixarr.index_axis(Axis(1),arang);
        let line=matrixarr.row(arang);
        let mut line1=line.clone().to_vec();
        let mline1  =&line1[..];
        // console_log!("line1 {:?} {:?}",comment,mline1);
        let line2:Vec<&str>=line.to_vec().iter().map(|e| match *e {
            0 => " ",
            1 => "a",
            2 => "b",
            4 => "c",
            8 => "d",
            16 => "e",
            32 => "f",
            64 => "g",
            128 => "h",

            _ =>"!"
        }).collect();
        line1.sort();
        line1.dedup();
        console_log!("l2{:?} {:?} {:?} {:?} {:?} {:?} {:?}",line2.join(""), line2.len(), arang, line1,height, code,comment);
       // console_log!("A:{:?} {:?} {:?}",line2,code,comment);
    }

}

pub fn checkCollisions(detectCollision: (Vec<CollisionsLine>, i32, i32, i32, i32), ds: Rc<RwLock<DocumentStorage>>, ps: Rc<Mutex<PaletteElements>>) -> Option<Vec<CollisionZone>> {
    const EL_COUNTER:u8=5;
    let (collisions, minx, miny, maxx, maxy) = detectCollision;
    let width = maxx - minx;
    let height = maxy - miny;
    let mut matrixArr: Array2<u128> = Array2::zeros((height as usize, width as usize));
    let mut counter = 0;
    let mut element_counter = 0;
    let mut elementsC: HashMap<u32, u32> = HashMap::new();
    let mut elementsX: HashMap<u32, i32> = HashMap::new();
    let mut vv = vec![];
    if let Ok(elements) = ps.lock() {
        let data = collisions.iter();
        counter = 0;
        for element in data
        {
            let element_kind = element.element_kind;
            if let Some(elel) = elements.iter().find(|e| {
                if let Ok(elem) = e.lock() {
                    if elem.GetKind() == element_kind { return true; }
                }
                false
            }) {
                if let Ok(elem) = elel.lock() {
                    let shiftx = element.startx - minx;
                    let shifty = element.starty - miny;
                    elementsC.insert(element_counter, element.id);
                    elementsX.insert(element.id, element.startx );
                    let config = elem.GetConfig();


                    let emptyMask=elem.GetBitMask(AllMasks::MaskEmpty);
                    let mask1 = emptyMask.clone(); //elem.GetBitMask(AllMasks::MaskMain) * 2_u128.pow(counter+0)*0;
                    let mask2 = if !config.withBody {  emptyMask.clone() } else { elem.GetBitMask(AllMasks::MaskBody) * 2_u128.pow(counter + 1) };
                    let mask3 = if !config.withInputs {  emptyMask.clone() } else { elem.GetBitMask(AllMasks::MaskInputs) * 2_u128.pow(counter + 2) };
                    let mask2mask2=mask3.clone();
                    let mask4 = if !config.withParameters { emptyMask.clone() } else { elem.GetBitMask(AllMasks::MaskParameters) * 2_u128.pow(counter + 3) };
                    let mask5 = if !config.withOutputs {  emptyMask } else { elem.GetBitMask(AllMasks::MaskOutputs) * 2_u128.pow(counter + 4)};
                    let mask4mask4=mask5.clone();
                    let mut mask22: Vec<u128> = mask2.iter().copied().collect();
                    mask22.sort();
                    mask22.dedup();
                    counter += EL_COUNTER as u32;
                    element_counter += 1; //element_counter+1;
                    let tmpmainmask=mask1.clone()+mask2.clone()+mask3.clone()+mask4.clone()+mask5.clone();
                    let mut  newextended=Array2::zeros((height as usize,width as usize));
                    for y in 0..tmpmainmask.shape()[0] {
                        for x in 0..tmpmainmask.shape()[1]
                        {
                            newextended[[y+shifty as usize,x+shiftx as usize]]=tmpmainmask[[y,x]];
                        }
                    }
                    matrixArr = matrixArr+newextended.clone(); // matrixArr + mask1_1.clone() + mask2_1.clone() + mask3_1.clone() + mask4_1.clone() + mask5_1.clone();
                    let mut matrixArr0: Array2<u128> = Array2::zeros((height as usize, width as usize));
                    matrixArr0 = matrixArr0 + newextended.clone() ; //mask1_1.clone() + mask2_1.clone() + mask3_1.clone() + mask4_1.clone() + mask5_1.clone();
                    let mut tmpVec = matrixArr0.clone().into_raw_vec();
                    let mut mutmask2=mask2mask2.into_raw_vec();
                    let mut mutmask4=mask4mask4.into_raw_vec();
                    mutmask2.sort();
                    mutmask2.dedup();
                    mutmask4.sort();
                    mutmask4.dedup();
                    tmpVec.dedup();
                    tmpVec.sort();
                    tmpVec.dedup();
                    vv.extend(tmpVec);
                }
            }
        }
        let mut finalVec = matrixArr.clone().into_raw_vec();
        finalVec.dedup();
        finalVec.sort();
        finalVec.dedup();
        for acounter in 0..counter {
            vv.push(2_u128.pow(acounter))
        }
        vv.sort();
        vv.dedup();
        let resres: Vec<u128> = finalVec.clone().into_iter().filter(|e| {
            if e == &(0 as u128) { return false; }
            if let Some(el) = vv.iter().find(|al| al == &e) {
                return false;
            }
            true
        }).collect();
        let mut allcrossings: Vec<u128> = vec![];
        let mut acounter = 0;
        for result in resres.iter() {
            let felements = getEls(*result, counter, EL_COUNTER as u32);
            let crossins = felements.0;
            allcrossings.extend(crossins);
            acounter += 1;
        }//
        allcrossings.sort();
        allcrossings.dedup();
        let mut pws = vec![];
        for line in allcrossings.iter() {
            let res1 = line.clone() as f64;
            let pow = (res1.ln() / 2_f64.ln()) as u8;
            pws.push(pow);
        }
        let mut collisions=vec![];
        for col in pws.iter() {
            let elIndex=col/ EL_COUNTER;
            let elSzero=col % EL_COUNTER;
            let element=elementsC.get(&(elIndex as u32)).unwrap();
            let elementX=elementsX.get(element).unwrap();
            collisions.push(CollisionZone{element_id: *element, element_x: *elementX, element_zone:match elSzero {
                0=>AllMasks::MaskMain,
                1=>AllMasks::MaskBody,
                2=>AllMasks::MaskInputs,
                3=>AllMasks::MaskParameters,
                4=>AllMasks::MaskOutputs,
                _ => AllMasks::MaskMain
            },

            });

        }
            return Some(collisions)
    }
    None
}
