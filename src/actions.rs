use yansi::Paint;

pub fn welcome() {
    println!("|-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-|");
    println!(
        "| {              }                                        |",
        Paint::red("Welcome to Rhonn")
    );
    println!("| This is a text-based experimental game.                 |");
    println!("| To play, simply read the prompt you are given,          |");
    println!("| and react accordingly by typing one word.               |");
    println!(
        "| If you want to quit, simply type {  }.                  |",
        Paint::green("exit")
    );
    println!("|-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-~-|");
    println!();
}
