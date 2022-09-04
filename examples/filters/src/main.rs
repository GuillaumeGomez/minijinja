use minijinja::value::Value;
use minijinja::{context, Environment, State};

fn slugify(_state: &State, value: String) -> String {
    value
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn get_nav(_state: &State) -> Value {
    vec![
        context! { href => "/", title => "Index" },
        context! { href => "/downloads", title => "Downloads" },
        context! { href => "/about", title => "About" },
    ]
    .into()
}

fn main() {
    let mut env = Environment::new();
    env.add_filter("slugify", slugify);
    env.add_function("get_nav", get_nav);
    env.add_template("template.html", include_str!("template.html"))
        .unwrap();

    let template = env.get_template("template.html").unwrap();
    let context = context! {
        page_title => "this is my page",
        year => 2022
    };

    for _ in 0..50000 {
        template.render(&context).unwrap();
    }
}
