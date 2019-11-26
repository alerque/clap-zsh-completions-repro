use clap::{App, Arg, Shell};

fn main() {
    let names = (b'a'..b'z')
        .map(|b| b as char)
        .flat_map(|char1| (b'a'..b'z').map(move |char2| format!("{}{}", char1, char2 as char)))
        .collect::<Vec<_>>();
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
    for (v, n) in values.iter().zip(&names) {
        app = app.arg(Arg::with_name(n).possible_value(v));
    }
    app.gen_completions_to(
        "clap-zsh-completions-repro",
        Shell::Zsh,
        &mut std::io::stdout(),
    );
}
