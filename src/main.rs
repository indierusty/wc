use std::fs::read_to_string;
use std::io::{stdin, Read};

// TODO: handle errors (eliminate .unwrap()'s)

#[derive(Debug, Clone)]
struct WC {
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
    name: String,
}

impl WC {
    fn init(name: String) -> Self {
        Self {
            bytes: 0,
            chars: 0,
            words: 0,
            lines: 0,
            name,
        }
    }

    fn read(name: String, data: &str) -> Self {
        let bytes = data.as_bytes().len();
        let chars = data.chars().count();
        let words = data.split_whitespace().count();
        let lines = data.lines().count();

        Self {
            bytes,
            chars,
            words,
            lines,
            name,
        }
    }

    fn print(&self, op: Options) {
        if !op.bytes && !op.chars && !op.bytes && !op.lines {
            print!(
                "{line:>6} {words:>6} {bytes:>6} {name}\n",
                line = self.lines,
                words = self.words,
                bytes = self.bytes,
                name = self.name
            );
            return;
        }

        if op.lines {
            print!("{:>6} ", self.lines);
        }
        if op.words {
            print!("{:>6} ", self.words);
        }
        if op.bytes {
            print!("{:>6} ", self.bytes);
        } else if op.chars {
            print!("{:>6} ", self.chars);
        }

        print!("{}\n", self.name);
    }
}

#[derive(Debug, Clone, Copy)]
struct Options {
    bytes: bool,
    chars: bool,
    words: bool,
    lines: bool,
    /// no FilePath/Stdin is provided
    nofiles: bool,
}

impl Options {
    fn init() -> Self {
        Self {
            bytes: false,
            chars: false,
            words: false,
            lines: false,
            nofiles: true,
        }
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut options = Options::init();
    let mut word_counts = Vec::new();

    for arg in args {
        match arg.as_str() {
            "-c" | "--bytes" => options.bytes = true,
            "-w" | "--words" => options.words = true,
            "-l" | "--lines" => options.lines = true,
            "-m" | "--chars" => options.chars = true,
            f => {
                options.nofiles = false;
                match f {
                    "-" => {
                        // Read Stdin
                        let wc = WC::read("-".to_string(), &read_stdin());
                        word_counts.push(wc);
                    }
                    ff => {
                        // Read File
                        let data = read_to_string(&ff).unwrap();
                        let wc = WC::read(f.to_string(), &data);
                        word_counts.push(wc);
                    }
                }
            }
        }
    }

    if options.nofiles {
        // Read Stdin
        let wc = WC::read("-".to_string(), &read_stdin());
        word_counts.push(wc);
    }

    let mut total = WC::init("Total".to_string());

    for wc in word_counts {
        total.bytes += wc.bytes;
        total.chars += wc.chars;
        total.lines += wc.lines;
        total.words += wc.words;

        wc.print(options);
    }
}
