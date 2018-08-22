// #![windows_subsystem = "windows"]

#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

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
                {styles}
            </head>

            <body>
                <div id="main" class="flex full three-600 twelve-960" />
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
            match serde_json::from_str(arg).unwrap() {
                init => {
                    println!("WebView finished loading, sending files");
                    let docs = get_docs();
                    render(webview, docs);
                },
                preview { content } => println!("App called for preview, render the following to HTML: {}", content)
            }
        }, userdata);
}

fn render<'a , T>(webview: &mut WebView<'a, T>, docs: Vec<Document>) {
    println!("Rendering with {:?}", docs);
    webview.eval(&format!("rpc.render({})", serde_json::to_string(&docs).unwrap()));
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn get_docs() -> Vec<Document> {
    vec![]
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    init,
    preview { content: String }
}