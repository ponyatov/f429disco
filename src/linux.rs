use f429disco::*;
mod readline;

fn main() {
    println!("{} ver:{}", ABOUT, VERSION);
    println!("(c) {} <{}> {} {}", AUTHOR, EMAIL, YEAR, LICENSE,);
    loop {
        let cmd = readline::readline(">>> ");
        println!("command was: <{}>",cmd);
    }
}
