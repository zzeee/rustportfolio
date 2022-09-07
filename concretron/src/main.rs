//! Takes 2 audio inputs and outputs them to 2 audio outputs.
//! All JACK notifications are also printed out.
use std::io;
use crossbeam_channel::bounded;
use std::convert::TryInto;

use std::time::{Duration, Instant};
use num_complex::Complex32;
use std::fs::File;

mod analys;
use crate::analys::{cc, findMaxInd, countMore,calculationLine, aos, maxsize, f64ArrStr};
use rand::prelude::*;
use rand::distributions::{Normal, Distribution};
use std::io::{Write, BufReader, BufRead, Error};


fn main() {
    let  outputS = File::create("./dt1");
    let  outputS2 = File::create("./dt2");
    if let Ok(mut output)=outputS {
        if let Ok(mut output2) = outputS2 {
            let (client, _status) =
                jack::Client::new("rust_jack_simple", jack::ClientOptions::NO_START_SERVER).unwrap();
            let mut frequency = 1800.0;
            let sample_rate = client.sample_rate();
           // exit();

            println!("ККК{:?}",sample_rate);
           // std::process::exit(0);

            let frame_tq = 1.0 / sample_rate as f64;
            let mut time = 0.0;

            let in_source_compare = client
                .register_port("rust_in_ll", jack::AudioIn::default())
                .unwrap();
            let mut in_analyze = client
                .register_port("rust_in_br", jack::AudioIn::default())
                .unwrap();
            let mut out_a = client
                .register_port("rust_out_l", jack::AudioOut::default())
                .unwrap();
            let mut out_port = client
                .register_port("sine_out", jack::AudioOut::default())
                .unwrap();
            let mut out_b = client
                .register_port("rust_out_r", jack::AudioOut::default())
                .unwrap();
            let mut counter = 300;
            let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
                let mut now = Instant::now();
                if counter == 0 {
                    return jack::Control::Quit
                }
                counter = counter - 1;
                // println!("count {:?}", counter);

                if counter < 200 {
                    // println!("PLAY!");
                    let mut out_a_p_source = out_a.as_mut_slice(ps);
                    let mut out_b_p_analyze = out_b.as_mut_slice(ps);
                    let in_a_p = in_source_compare.as_slice(ps);
                    out_a_p_source.clone_from_slice(&in_a_p);
                    let in_b_p = in_analyze.as_slice(ps);
                    out_b_p_analyze.clone_from_slice(&in_b_p);
                    // println!("eee {:?}", out_b_p_analyze.len());

                    // let ttd=String::from(in_b_p);
                    let ares = f64ArrStr(in_b_p);

                    write!(output, "L_in_b_p {:?}", in_b_p).unwrap();
                    write!(output2, "L_in_a_p {:?}", in_a_p).unwrap();


                    let spectrum = microfft::real::rfft_256(&mut out_b_p_analyze);

                    let spectrum2 = microfft::real::rfft_256(&mut out_a_p_source);


                   println!("{:?}",spectrum2);
                    let amplitudes_analyze: Vec<_> = spectrum.iter().map(|c| c.norm() as f64).collect();
                    let amplitudes_source: Vec<_> = spectrum2.iter().map(|c| (c.norm() as f64 / 4.)).collect();
                    // println!("elp0-1 {:?} ", now.elapsed().as_micros() );

                    let analyze_Vec: calculationLine = amplitudes_analyze.try_into().unwrap();
                    let source_bVec: calculationLine = amplitudes_source.try_into().unwrap();
                    //println!("elp0-2 {:?} ", now.elapsed().as_micros() );

                    // let cc_res0=cc(out_b_p_analyze,out_a_p_source);
                    let cc_res = cc(&analyze_Vec, &source_bVec);
                    // println!("elp0-3 {:?} ", now.elapsed().as_micros() );

                    println!("{:?}",cc_res);
                    let mind = findMaxInd(cc_res);
                    let cmore = countMore(cc_res[aos], cc_res);
                    // println!("elp1 {:?} ", now.elapsed().as_micros() );

                    if mind != aos {
                        println!("SS4 {:?} {:?}  {:?} {:?} {:?}", mind, aos, cc_res[mind], cc_res[aos], cmore);
                    }
                    let out = out_port.as_mut_slice(ps);

                    /*  while let Ok(f) = rx.try_recv() {
                time = 0.0;
                frequency = f;
                println!("try_recv {:?}", frequency);
            }*/
                    for v in out.iter_mut() {
                        let x = frequency * time * 3.0 * std::f64::consts::PI;
                        let y = x.sin();
                        *v = y as f32;
                        time += frame_tq;
                    }
                    println!("elp_f {:?} ", now.elapsed().as_micros());
                }


                jack::Control::Continue
            };
            let process = jack::ClosureProcessHandler::new(process_callback);
            // Activate the client, which starts the processing.
            let active_client = client.activate_async(Notifications, process).unwrap();
            let client_analyzer = active_client.as_client().port_by_name("rust_jack_simple:rust_in_br").unwrap();
            let client_comparator = active_client.as_client().port_by_name("rust_jack_simple:rust_in_ll").unwrap();
            let sine = active_client.as_client().port_by_name("rust_jack_simple:sine_out").unwrap();
            let client_in = active_client.as_client().port_by_name("system:capture_1").unwrap();
          //  let client_in2 = active_client.as_client().port_by_name("system:capture_1").unwrap();
            let client_out = active_client.as_client().port_by_name("system:playback_1").unwrap();
            active_client.as_client().connect_ports(&client_in, &client_analyzer);
         //   active_client.as_client().connect_ports(&client_in2, &client_analyzer);
            active_client.as_client().connect_ports(&sine, &client_out);
            active_client.as_client().connect_ports(&sine, &client_comparator);
            // Wait for user input to quit
            println!("Press enter/return to quit...");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).ok();
            active_client.deactivate().unwrap();
        }
    }
}

struct Notifications;

impl jack::NotificationHandler for Notifications { }
