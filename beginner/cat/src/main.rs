use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::iter;
const SMALL_BUFFER_SIZE: usize = 256;
const LARGE_BUFFER_SIZE: usize = 64 * 1024;

fn redirect_stream<R, W>(reader: &mut R, writer: &mut W, buffer: &mut Vec<u8>) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    loop {
        let len_read = (reader.read(buffer))?;

        if len_read == 0 {
            return Ok(());
        }

        (writer.write_all(&buffer[..len_read]))?;

        if len_read == buffer.len() && len_read < LARGE_BUFFER_SIZE {
            buffer.extend(iter::repeat(0).take(len_read));
        }
    }
}

fn handle_arg<R, W>(reader: &mut R, writer: &mut W, buffer: &mut Vec<u8>)
where
    R: Read,
    W: Write,
{
    if let Err(err) = redirect_stream(reader, writer, buffer) {
        if let Err(io_err) = writeln!(&mut io::stderr(), "Error: {}", err) {
            panic!("Unable to write to stderr: {}", io_err);
        }
    }
}

fn main() {
    let mut args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        args.push("-".into());
    }

    let stdout = &mut io::stdout();
    let buffer = &mut vec![0; SMALL_BUFFER_SIZE];

    for arg in args {
        if arg == "-" {
            handle_arg(&mut io::stdin(), stdout, buffer);
            continue;
        }

        println!("{}", arg);

        match File::open(arg) 
        {
            Ok(ref mut file) => handle_arg(file, stdout, buffer),
            Err(err) =>
            {
                if let Err(io_err) = writeln!(&mut io::stderr(), "Error iniital: {}", err) {
                    panic!("Unable to write to stderr: {}", io_err);
                }
                continue;
            }
        }
    }
}
