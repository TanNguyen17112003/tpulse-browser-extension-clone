use std::fmt;
use std::io::{self, Read, Write};
use ipipe::Pipe;

enum Error {
    Io(io::Error),
    NoMoreInput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "io error: {}", err),
            Error::NoMoreInput => write!(f, "the input stream reached the end"),
        }
    }
}

fn read_input<R: Read>(mut input: R) -> Result<String, Error> {
    let mut buffer_size = [0; 4];
    match input
        .read_exact(&mut buffer_size)
        .map(|()| u32::from_ne_bytes(buffer_size))
    {
        Ok(size) => {
            let mut buffer = vec![0; size as usize];
            input.read_exact(&mut buffer).unwrap();
            let value = String::from_utf8(buffer).unwrap();
            Ok(value)
        }
        Err(e) => match e.kind() {
            io::ErrorKind::UnexpectedEof => Err(Error::NoMoreInput),
            _ => Err(Error::Io(e)),
        },
    }
}

fn main() {
    let mut pipe = Pipe::with_name("tpulse_pipe").expect("Could not create named pipe for tpulse application");
    loop {
        match read_input(io::stdin()) {
            Ok(value) => 
            {
                let payload = value.to_string();
                writeln!(&mut pipe, "{}", payload).expect("Could not write data into the pipe to send");
                io::stderr().write_all(value.as_bytes()).unwrap()
            },
            Err(e) => {
                if let Error::NoMoreInput = e {
                    break;
                }
                eprintln!("{}", format!("{{ \"error\": \"{}\" }}", e));
            }
        }
    }
}
