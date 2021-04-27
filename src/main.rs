use std::{env, fs, io::Read, sync::mpsc, thread};

use markdown_viewer::watch;
use notify::{Op, RawEvent};

fn main() {
    let path = env::args().nth(1).expect("Arg 1 mst be a path");

    // if let Err(e) = watch(path) {
    //     println!("error: {:?}", e);
    // }

    // let (tx, rx) = mpsc::channel();

    // if let Err(e) = watch(tx, path) {
    //     println!("error: {:?}", e);
    // }

    // let msg = rx.recv().unwrap();
    // println!("{:#?}", msg);

    // match watch(tx, path) {
    //     // Ok(path) => loop {
    //     //     match rx.recv() {
    //     //         // Ok(RawEvent {
    //     //         //     op: Ok(Op::WRITE),
    //     //         //     path: Some(file_written),
    //     //         //     ..
    //     //         // }) if file_written == path => {
    //     //         //     println!("wrote file!");
    //     //         // },
    //     //         Ok(ev) => println!("{:#?}", ev),
    //     //         Err(e) => panic!("{:#?}", e),
    //     //     }
    //     // },
    //     Ok(path) => println!("{:#?}", rx.recv()),
    //     Err(e) => println!("error watching file: {:?}", e),
    // }

    // match watch(tx, path) {
    //     Ok(_) => {
    //         println!("does this even run");
    // while let Ok(ev) = rx.try_recv() {
    //     println!("{:#?}", ev);
    // }
    //     },
    //     Err(e) => println!("error creating watcher: {:?}", e),
    // }

    // let (tx, rx) = mpsc::channel();
    // watch(tx, path).unwrap();

    // let m = rx.recv().unwrap();
    // println!("{:?}", m);

    watch(path, |path, data| match data {
        Ok(RawEvent {
            op: Ok(Op::WRITE),
            path: Some(file_written),
            ..
        }) if file_written == path => {
            let contents =
                fs::read_to_string(&file_written).expect("unable to read file");

            let mut ops = comrak::ComrakOptions::default();
            ops.render.escape = true;

            let html = comrak::markdown_to_html(&contents, &ops);

            println!("{}", html);
        },

        Ok(_) => {},

        Err(e) => println!("error: {:?}", e),
    })
    .unwrap();
}
