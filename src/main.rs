use text_io::read;
use yansi::Paint;

fn main() {
    println!("Hello, world!");
    let mut playing = true;
    let mut save: String = String::new();

    while playing {

        let action: String = read!();

        if action == "exit" {
            println!("{}", Paint::blue("Thank you for playing Rhonn"));
            println!("Here is your save:{}", Paint::green(&save));
            playing = false;

        } else {
            save.push_str(" ");
            save.push_str(&action);
            println!("Action performed: {}", Paint::green(action));
        }

    }
}
