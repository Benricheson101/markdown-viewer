mod watch;

use std::{env, fs, path::PathBuf, thread};

use comrak::ComrakOptions;
use nfd::Response;
use notify::{Op, RawEvent};
use watch::watch;
use web_view::Content;

fn main() {
    let path = if let Some(path) = env::args().nth(1) {
        let path = PathBuf::from(path);

        if !path.exists() {
            panic!("file does not exist");
        }

        path
    } else {
        if let Response::Okay(path) =
            nfd::open_file_dialog(Some("md"), Some(".")).unwrap()
        {
            path.into()
        } else {
            panic!("unknown file")
        }
    };

    let html = update_html(&path);

    let wv = web_view::builder()
        .title("Markdown Viewer")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()));

    let wv = if cfg!(debug_assertions) {
        wv.debug(true)
    } else {
        wv
    };

    let wv = wv.build().unwrap();

    let handle = wv.handle();

    thread::spawn(move || {
        watch(path, |path, data| match data {
            Ok(RawEvent {
                op: Ok(Op::WRITE),
                path: Some(file_written),
                ..
            }) if file_written == path => {
                handle
                    .dispatch(move |webview| {
                        webview.set_html(&update_html(&path))
                    })
                    .unwrap();
            },

            Ok(_) => {},

            Err(e) => panic!("error: {:?}", e),
        })
        .unwrap();
    });

    wv.run().unwrap();
}

fn update_html(path: &PathBuf) -> String {
    let contents = fs::read_to_string(path).expect("unable to read file");

    format!(
        include_str!("../static/index.html"),
        md = render_markdown(&contents),
        styles = include_str!("../static/style.css"),
    )
}

fn render_markdown(md: &str) -> String {
    let mut ops = ComrakOptions::default();
    ops.render.escape = true;
    ops.extension.tasklist = true;
    ops.extension.strikethrough = true;
    ops.extension.table = true;
    ops.extension.autolink = true;
    ops.extension.tasklist = true;

    comrak::markdown_to_html(md, &ops)
}
