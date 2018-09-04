#![windows_subsystem = "windows"]

extern crate glob;
extern crate pulldown_cmark;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use glob::glob;
use pulldown_cmark::{html, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES, Parser};
use std::fs;
use web_view::*;

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
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.10.0-rc/dist/katex.min.css" integrity="sha384-D+9gmBxUQogRLqvARvNLmA9hS2x//eK1FhVb9PiU86gmcrBrJAQT8okdJ4LMp2uv" crossorigin="anonymous">
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

                <script defer src="https://cdn.jsdelivr.net/npm/katex@0.10.0-rc/dist/katex.min.js" integrity="sha384-ttOZCNX+557qK00I95MHw9tttcgWn2PjR/bXecuEvENq6nevFtwSSQ6bYEN6AetB" crossorigin="anonymous"></script>
                <script defer src="https://cdn.jsdelivr.net/npm/katex@0.10.0-rc/dist/contrib/auto-render.min.js" integrity="sha384-yACMu8JWxKzSp/C1YV86pzGiQ/l1YUfE8oPuahJQxzehAjEt2GiQuy/BIvl9KyeF" crossorigin="anonymous"></script>
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

            // println!("Rendering with {:?}", arg);
            match serde_json::from_str(arg).unwrap() {
                init => {
                    println!("WebView finished loading, sending files");
                    let docs = get_docs();
                    render(webview, docs);
                },
                preview { contents } => {
                    println!("Received preview request");
                    // println!("App called for preview, render the following to HTML: {}", contents);
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
    let parsed = parse_markdown(&contents);
    // println!("Sending formatted doc: {}", parsed);
    webview.eval(&format!("rpc.renderPreview({})", serde_json::to_string(&parsed).unwrap()));
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn parse_markdown(s: &str) -> Document {
    let parser = Parser::new_ext(s, OPTION_ENABLE_FOOTNOTES | OPTION_ENABLE_TABLES);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    return Document { path: String::new(), contents: html_buf };
}

fn get_docs() -> Vec<Document> {
    let mut docs: Vec<Document> = vec![];

    let mut root = "./*.md";
    if cfg!(debug_assertions) {
        root = r#"D:\Drive\PhD\Notes\009 Annotated Bibliography\*.md"#;
    }

    for path in glob(root).expect("Failed to read glob pattern for files") {
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