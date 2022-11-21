use text_io::read;
use yansi::Paint;

mod actions;

fn main() {
    actions::welcome();
    let mut playing = true;
    let mut actions: Vec<String> = Vec::new();



    while playing {

        let action: String = read!();

        if action == "exit" {
            println!("{}", Paint::blue("Thank you for playing Rhonn"));
            let join = actions.join(" ");
            println!("Here is your save (copy this): {}", Paint::green(join));
            playing = false;

        } else {
            println!("Action performed: {}", Paint::green(&action));
            actions.push(action);
        }

    }
}