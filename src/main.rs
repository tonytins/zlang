use clap::Clap;
use zc::parser::parse;

#[derive(Clap)]
#[clap(author, about, version)]
struct Opts {
    #[clap(short, long, default_value = "sample.jax")]
    pub file: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let unparsed_file = std::fs::read_to_string(opts.file.as_str())
        .expect("Failed to read jax file");
    let parser = parse(&unparsed_file)
        .expect("Failed to parse file.");

    println!("{:?}", &parser);
}
