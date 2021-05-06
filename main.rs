use std::env;
use std::env::args;
use std::fs::File;
use std::io::{Result, BufReader, BufRead};

fn main() {
    let mut reverse = false;
    let mut arguments: Vec<String> = env::args().collect();
    
    if arguments[1].contains("-r"){
        &arguments.remove(1);
        reverse = true;
    }
    let mut Order:  Vec<String> = vec![];

    for arg in arguments.iter().skip(1){
        let mut file = File::open(arg)
            .expect("File does not exist");
        let Buff = BufReader::new(&mut file);
        let mut words: Vec<_> = Buff.lines().map(|l| l.expect("File does not exist")).collect();
        Order.append(&mut words);
    }
    if reverse {
        Order.sort();
        Order.reverse();
        for line in Order {
            println!("{:?}", line);
        }
    }else{
        Order.sort();
        for line in Order {
            println!("{:?}", line);
        }
    }
}