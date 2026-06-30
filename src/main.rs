extern crate getopts;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn format_output(nrOfLines: &u64, nrOfWords: &u64, nrOfBytes: &u64) { 
    println!("+{}+{}+{}+", str::repeat("-", 9), str::repeat("-", 9), str::repeat("-", 9));
    println!("| {lines:<8}| {words:<8}| {bytes:<8}|", lines="Lines", words="Words", bytes="Bytes");
    println!("+{}+{}+{}+", str::repeat("-", 9), str::repeat("-", 9), str::repeat("-", 9));
    println!("| {lines:<8}| {words:<8}| {bytes:<8}|", lines=nrOfLines, words=nrOfWords, bytes=nrOfBytes);
    println!("+{}+{}+{}+", str::repeat("-", 9), str::repeat("-", 9), str::repeat("-", 9));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut hasFileInput = false;

    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() {
        let mut nrOfLines: u64 = 0;
        let mut nrOfWords: u64 = 0;       
	let mut nrOfBytes: u64 = 0; 

        for line in std::io::stdin().lines() {
            let currLine = line.unwrap(); 
	    nrOfLines += 1;
	    nrOfWords += currLine.split_whitespace().collect::<Vec<_>>().len() as u64;
	    nrOfBytes += (currLine.len() + 1) as u64;
	}

        format_output(&nrOfLines, &nrOfWords, &nrOfBytes);
    }
    
    hasFileInput = true;

}
