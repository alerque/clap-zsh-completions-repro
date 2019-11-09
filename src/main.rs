use clap::{App, Arg, Shell};

fn main() {
    let values = (0..128u8)
        .map(|i| i as char)
        .filter(|c| !c.is_ascii_control())
        .map(|c| c.to_string())
        .chain(
            ["&*", "$@", "$#", "$?", "$-", "$$", "$!", "$0", "$_"]
                .iter()
                .map(|s| s.to_string()),
        )
        .collect::<Vec<_>>();

    let mut app = App::new("clap-zsh-completions-repro");
    for v in &values {
        app = app.arg(Arg::with_name("a").possible_value(v));
    }
    app.gen_completions_to(
        "clap-zsh-completions-repro",
        Shell::Zsh,
        &mut std::io::stdout(),
    );
}
