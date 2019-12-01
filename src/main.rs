use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // println!("Hello, world!");

    // let pro_name = std::env::args().nth(0);
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::from_args();
    // println!("your pattern is: {}", args.pattern);
    // println!("your path is: {}", args.path);

    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file bud");

    assert_eq!(3, 3);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // let result = std::fs::read_to_string("bash.txt");
    // match result {
    //     Ok(content) => { println!("File content: {}", content); }
    //     Err(error) => { println!("Oh noes: {}", error); }
    // }

}

fn read_file() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string("bash.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };

    println!("file content: {}", content);
    Ok(())

}

