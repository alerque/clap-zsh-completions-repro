use clap::{Arg, App, Shell};

fn main() {
    let mut app = App::new("clap-zsh-completions-repro")
        .arg(Arg::with_name("fail").possible_value("includes-pipe|"));
    app.gen_completions_to("clap-zsh-completions-repro", Shell::Zsh, &mut std::io::stdout());
}
