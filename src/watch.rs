use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{self, RecvError},
};

use notify::{RawEvent, RecommendedWatcher, RecursiveMode, Watcher};

// pub fn watch<P: Into<PathBuf> + Clone>(
//     path: P,
// ) -> notify::Result<(Receiver<RawEvent>, PathBuf)> {
// pub fn watch<P>(tx: Sender<RawEvent>, path: P) -> notify::Result<()>
// where
//     P: Clone + Into<PathBuf>,
// {
//     let path: PathBuf = path.into();
//     let path = fs::canonicalize(path)?;

//     // let (send, recv) = mpsc::channel();
//     let mut watcher: RecommendedWatcher = Watcher::new_raw(tx)?;

//     if path.is_dir() {
//         panic!("cannot watch directory");
//     }

//     // required for compatibility with some editors.
//     //
//     // instead of writing directly to the file, some editors write to a
//     // temporary file, remove the original file and then move the temp file
// to     // the location of the original file. in vim, this behavior is
// controlled     // under the `backupcopy` option
//     let dir = path.parent().unwrap();

//     println!("watching {}", dir.display());
//     watcher.watch(dir, RecursiveMode::NonRecursive)?;

//     Ok(())

//     // loop {
//     //     match rx.recv() {
//     //         Ok(RawEvent {
//     //             op: Ok(Op::WRITE),
//     //             path: Some(file_written),
//     //             ..
//     //         }) if file_written == path => {
//     //             println!("wrote file!");
//     //         },
//     //         _ => {},
//     //     }
//     // }

//     // loop {
//     //     let msg = recv.recv().unwrap();
//     //     dbg!(&msg);

//     //     tx.send(msg).unwrap();
//     // }
// }

// TODO: figure out why channels werent working here

/// Watch a file for updates with `notify`
pub fn watch<P, F>(path: P, f: F) -> notify::Result<()>
where
    P: Clone + Into<PathBuf>,
    F: Copy + FnOnce(PathBuf, Result<RawEvent, RecvError>),
{
    let path: PathBuf = path.into();
    let path = fs::canonicalize(path)?;

    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new_raw(tx)?;

    if path.is_dir() {
        panic!("cannot watch directory");
    }

    // required for compatibility with some editors.
    //
    // instead of writing directly to the file, some editors write to a
    // temporary file, remove the original file and then move the temp file to
    // the location of the original file. in vim, this behavior is controlled
    // under the `backupcopy` option
    let dir = path.parent().unwrap();

    watcher.watch(dir, RecursiveMode::NonRecursive)?;

    loop {
        f(path.clone(), rx.recv());
    }
}
