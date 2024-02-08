use std::env;

fn main() {

    // let args:Vec<String> = env::args().skip(1).collect();

    // if args.len()==0
    // {
    //     println!("");
    // }
    // else
    // {
    //     println!("{}", args.join(" "));
    // }

    let mut args = env::args().skip(1);

    if let Some(arg) = args.next() {
        print!("{}", arg);

        for arg in args
        {
            print!(" {}", arg);
        }
    }

}
