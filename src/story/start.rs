use textwrap_macros::dedent;

pub struct Action {
    // Type here
    word: String,
    event: String,
}

pub struct Step {
    words: String,
    prompt: String,
}

pub struct Out {
    word: String,
    target: Scene,
}

pub struct Scene {
    // Describes the scene
    pub description: String,
    steps: [Step; 3],
}


pub fn makeStartScene () -> Scene {
    let START: Scene = Scene {
        description: dedent!(r#"
                "Greetings, brave warrior! You look tough, but can you add 1 plus 1?"
            "#).to_string(),
        steps: [
            Step {
                words: String::from("two"),
                prompt: dedent!(
                    r###"
    
                    You are correct, indeed! I am Gulgugugla the Wise. You shall be hearing of me a lot.
                    
                    In order to receive my blessing you must give me a hug.
    
                    "###
                ).to_string(),
            },
            Step {
                words: String::from("hug"),
                prompt: dedent!(
                    r###"
    
                    You hug the old man.
                    "A splendid hug! I shall grant you my blessing.", he says.
                    He raises his palms to the sky and sings a chant.
                    "There. You may `leave` through the door behind you."
    
                    "###
                ).to_string(),
            },
            Step {
                words: String::from("leave"),
                prompt: dedent!(
                    r###"
    
                    You leave the old man's hut through the door, towards new adventure. 🤠
    
                    "###
                ).to_string(),
            },
        ],
    };
    return START;
}
