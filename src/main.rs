//#![windows_subsystem = "windows"]

extern crate glob;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use web_view::*;
use glob::glob;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    path: String,
    contents: String
}

fn main() {
    println!("LOADING APPLICATION");

    let html = format!(r#"
        <!doctype html>
        <html>
            <head>
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                {styles}
            </head>

            <body>
                <div id="main" class="wrapper" />
                {templates}

                <!--[if lt IE 9]>
                <div class="ie-upgrade-container">
                    <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
                    <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
                </div>
                <!--[endif]-->
                <!--[if gte IE 9 | !IE ]> <!-->
                {scripts}
                <!--[endif]-->
            </body>
        </html>
        "#,
        styles = inline_style(include_str!("../www/styles.css")),
        templates = include_str!("../www/templates.html"),
        scripts = inline_script(include_str!("../www/lib.js")) + &inline_script(include_str!("../www/index.js"))
    );

    // fs::write("./test.html", &html).unwrap();
    // println!("{}", html);

    let size = (800, 600);
    let resizable = true;
    let debug = true;

    let init_cb = |_: MyUnique<WebView<Vec<Document>>>| ();

    let userdata = vec![];

    run("Markdown Viewer",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        init_cb,
        |webview, arg, _| {
            use Cmd::*;

            println!("Rendering with {:?}", arg);
            match serde_json::from_str(arg).unwrap() {
                init => {
                    println!("WebView finished loading, sending files");
                    let docs = get_docs();
                    render(webview, docs);
                },
                preview { contents } => {
                    println!("App called for preview, render the following to HTML: {}", contents);
                    render_preview(webview, contents);
                }
            }
        }, userdata);
}

fn render<'a , T>(webview: &mut WebView<'a, T>, docs: Vec<Document>) {
    // println!("Rendering with {:?}", docs);
    webview.eval(&format!("rpc.render({})", serde_json::to_string(&docs).unwrap()));
}

fn render_preview<'a, T>(webview: &mut WebView<'a, T>, contents: String) {
    webview.eval(&format!("rpc.renderPreview({})", contents));
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn get_docs() -> Vec<Document> {
    let mut docs: Vec<Document> = vec![];

    for path in glob(r#"D:\Drive\PhD\Notes\009 Annotated Bibliography\*.md"#).expect("Failed to read glob pattern for files") {
        match path {
            Ok(path) => docs.push(Document {
                path: path.as_path().as_os_str().to_str().unwrap().to_string(),
                contents: fs::read_to_string(path).unwrap()
            }),
            Err(e) => println!("Error reading path: {:?}", e)
        }
    }

    return docs
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    init,
    preview { contents: String }
}