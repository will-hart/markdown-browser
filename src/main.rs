#![windows_subsystem = "windows"]

extern crate glob;
extern crate pulldown_cmark;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use glob::glob;
use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::fs;
use std::io;
use std::path::Path;
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

                <script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/katex.min.js" integrity="sha256-q01RVaHUJiYq9aq0FwkI6GAmMtOmRgToK8aEHHm4Xl8=" crossorigin="anonymous"></script>
                <script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/contrib/auto-render.min.js" integrity="sha256-CiPSQ9n316ms9u5yYJ092wI+FeybXvesfbnOUvSRvYA=" crossorigin="anonymous"></script>
            </body>
        </html>
        "#,
        styles = inline_style(include_str!("../www/styles.css")),
        templates = include_str!("../www/templates.html"),
        scripts = inline_script(include_str!("../www/lib.js")) + &inline_script(include_str!("../www/index.js"))
    );

    // fs::write("./test.html", &html).unwrap();
    // println!("{}", html);

    web_view::builder()
        .title("Markdown Viewer")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(vec![])
        .invoke_handler(|webview, arg| {
            use Cmd::*;

            match serde_json::from_str(arg).unwrap() {
                Init => {
                    println!("INIT");
                    let docs = get_docs();
                    render(webview, docs)
                },
                Preview { contents } => {
                    println!("PREVIEW");
                    render_preview(webview, contents)
                },
            }
        })
        .build()
        .unwrap()
        .run()
        .unwrap();
}

fn render(webview: &mut WebView<Vec<Document>>, docs: Vec<Document>) -> WVResult {
    let render_tasks = {
        format!("rpc.render({})", serde_json::to_string(&docs).unwrap())
    };

    webview.eval(&render_tasks)
}

fn render_preview(webview: &mut WebView<Vec<Document>>, contents: String) -> WVResult {
    let render_tasks = {
        let parsed = parse_markdown(&contents);
        println!("Sending formatted doc: {}", parsed.contents);
        format!("rpc.renderPreview({})", serde_json::to_string(&parsed).unwrap())
    };

    webview.eval(&render_tasks)
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn parse_markdown(s: &str) -> Document {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(s, options);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    return Document { path: String::new(), contents: html_buf };
}

fn get_folder_path() -> io::Result<String> {
    let app_dir = env::current_dir();
    let app_dir = match app_dir {
        Result::Ok(s) => format!("{}", s.as_os_str().to_str().unwrap()),
        Result::Err(_) => format!("{}", "./*.md"),
    };

    let folder_config = format!("{}\\folder.txt", app_dir);

    if Path::new(&folder_config).exists() {
        println!("Path {} exists, reading", folder_config);
        let result = fs::read_to_string(folder_config);
        return match result {
            Result::Err(_) => Ok(format!("{}", "./*.md")),
            Result::Ok(s) => Ok(format!("{}", s)),
        };
    }

    return Ok(format!("{}", "./*.md"));
}

fn get_docs() -> Vec<Document> {
    let mut docs: Vec<Document> = vec![];
    let root = get_folder_path().unwrap();

    println!("Reading files at {}", root);

    for path in glob(&root).expect("Failed to read glob pattern for files") {
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
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Preview { contents: String }
}
