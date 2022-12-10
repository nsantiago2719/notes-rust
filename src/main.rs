use std::io;

struct Note {
    title: String,
    description: String,
    id: u32,
}

impl Note {
    fn new(title: &str, description: &str) -> Note {
        Note {
            title: title.to_string(),
            description: description.to_string(),
            id: fastrand::u32(..)
        }
    }
}

fn main() {
    let mut notes = Vec::new();
    loop {
        let mut options_ans = String::new();
        options_ans = get_input_value("Add new note to your list?(y/n)");

        if options_ans == "y" {
            let mut title = String::new();
            let mut description = String::new();

            title = get_input_value("Enter note title: ");
            description = get_input_value("Enter note description: ");

            let note = Note::new(&title, &description);
            notes.push(note);
        } else {
            break;
        }
    }

    print!("{esc}c", esc = 27 as char);
    println!("This are your notes you entered");
    println!("------------------------------------------------------------------------");

    for n in notes {
        println!("Title: {}", n.title);
        println!("Description: {}", n.description);
        println!("------------------------------------------------------------------------");
    }
    
    
}

fn get_input_value(d: &str) -> String {
    let mut value = String::new();
    println!("{}: ", d);
    io::stdin().read_line(&mut value).expect("Failed to get value");
    trim_newline(&value)
}

fn trim_newline(s: &String) -> String {
    s.trim_end_matches(&['\r', '\n']).to_string()
}
