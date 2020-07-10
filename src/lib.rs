mod printing;
use printing::{
    move_print_questions, print_answer_options, print_boxed, print_enumerated,
    print_enumerated_answers, print_line, print_questions, print_to_file,
};
use std::error::Error;

pub struct Sentence {
    pub initial_sentence: String,
    pub splits: Vec<Vec<String>>,
    pub current_split: usize,
    pub answers: Vec<Vec<AnswerOption>>,
    pub completed: bool,
}

#[derive(Debug)]
pub struct AnswerOption {
    pub is_question: bool,
    pub mark: u8,
    pub answer: String,
    pub feedback: String,
}

impl Sentence {
    fn new(entry: String) -> Self {
        let mut splits: Vec<Vec<String>> = Vec::new();
        let mut words: Vec<String> = Vec::new();
        let answers: Vec<Vec<AnswerOption>> = Vec::new();
        for word in entry.split_whitespace() {
            words.push(String::from(word));
        }
        splits.push(words);
        Self {
            initial_sentence: entry,
            splits,
            current_split: 0,
            answers,
            completed: false,
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut questions: Vec<Sentence> = Vec::new();

    loop {
        // clear the screen
        print!("\x1B[2J");
        println!("");
        print_boxed("Sentences into Greek");
        println!(" ~i: incomplete ~D: done");
        print_questions(&questions);
        print_boxed(&format!(
            "{:20}{:20}{:20}{:16}\n{:20}{:20}{:20}",
            "Add question: a",
            "Edit question: e",
            "Delete question: d",
            "Move question: m",
            "Print to file: p",
            "Start again: s",
            "Quit: q"
        ));
        match read_input().as_str() {
            "a" => enter_question(&mut questions),
            "e" => {
                let choice = get_num_choice("Enter no.: ");
                let chosen_question = &mut questions[choice];
                process_question(chosen_question);
            }
            "d" => {
                let choice = get_num_choice("Enter no.: ");
                questions.remove(choice);
            }
            "m" => {
                let choice = get_num_choice("Enter no.: ");
                move_question_dialog(&mut questions, choice);
            }
            "p" => print_to_file(&questions),
            "s" => {
                questions.clear();
                continue;
            }
            "q" => break,
            _ => continue,
        }
    }
    Ok(())
}

fn move_question_dialog(bank: &mut Vec<Sentence>, mut idx: usize) {
    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Move question");
        move_print_questions(bank, idx);
        print_boxed("Move up: u    Move down: d    Accept: a");
        match read_input().as_str() {
            "u" => {
                bank[idx - 1..idx + 1].rotate_right(1);
                idx -= 1;
            }
            "d" => {
                bank[idx..idx + 2].rotate_left(1);
                idx += 1;
            }
            "a" => break,
            _ => continue,
        }
    }
}

fn enter_question(bank: &mut Vec<Sentence>) {
    let mut sentence: Sentence = get_sentence();
    print_boxed("Return to menu: a    Process this one: p");
    match read_input().as_str() {
        "a" => bank.push(sentence),
        "p" => {
            process_question(&mut sentence);
            bank.push(sentence);
        }
        _ => {}
    }
}

fn process_question(sentence: &mut Sentence) {
    edit_sentence(sentence);
    set_answers(sentence);
}

fn read_input() -> String {
    let mut rl = rustyline::Editor::<()>::new();
    let readline = rl.readline(">> ");
    readline.unwrap_or_default()
}

fn get_num_choice(prompt: &str) -> usize {
    loop {
        let mut rl = rustyline::Editor::<()>::new();
        let readline = rl.readline(prompt);
        let choice = readline.unwrap_or_default();
        let idx = choice.parse::<usize>();
        match idx {
            Ok(num) => return num - 1,
            Err(_error) => {
                println!("You must enter a number.");
                continue;
            }
        }
    }
}

fn revert_splits(sentence: &mut Sentence) {
    if sentence.current_split > 0 {
        sentence.current_split -= 1;
        sentence.splits.pop();
    }
}

fn get_sentence() -> Sentence {
    loop {
        print_boxed("Please enter your sentence.");
        println!("");
        let sentence = Sentence::new(read_input());
        println!("You entered: {}", sentence.initial_sentence);
        println!("");
        print_boxed("Continue: c    Replace: r");
        match read_input().as_str() {
            "r" => continue,
            _ => return sentence,
        };
    }
}

fn edit_sentence(sentence: &mut Sentence) -> &mut Sentence {
    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Edit the sentence.");
        print_enumerated(&sentence.splits[sentence.current_split]);
        print_boxed("Reorder: r    Join: j    Accept: a");
        match read_input().as_str() {
            "j" => {
                join_parts(sentence);
                continue;
            }
            "r" => {
                reorder_parts(sentence);
                continue;
            }
            _ => return sentence,
        }
    }
}

fn join_parts(sentence: &mut Sentence) -> &mut Sentence {
    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Join the words.");
        print_enumerated(&sentence.splits[sentence.current_split]);
        print_boxed(
            "Enter a word's number to join it with the next word.\nAccept: a     Revert: r",
        );
        let entry = read_input();
        match entry.as_str() {
            "a" => return sentence,
            "r" => revert_splits(sentence),
            "" => continue,
            _ => {
                let mut num = entry.parse::<usize>().unwrap();
                println!("You entered: {}", num);
                num -= 1;
                apply_join(sentence, num);
            }
        }
    }
}

fn apply_join(sentence: &mut Sentence, idx: usize) {
    if idx + 2 > sentence.splits[sentence.current_split].len() {
        println!("You cannot join the last word to a 'next word'. There is no 'next word'!");
        return;
    }
    let mut new_split: Vec<String> = Vec::new();
    let prev_split = &sentence.splits[sentence.current_split];
    let max = prev_split.len();
    for word in prev_split[..idx].iter() {
        new_split.push(word.clone());
    }
    new_split.push(format!("{} {}", prev_split[idx], prev_split[idx + 1]));
    if idx + 2 < max {
        for word in prev_split[idx + 2..].iter() {
            new_split.push(word.clone());
        }
    }
    sentence.splits.push(new_split);
    sentence.current_split += 1;
}

fn reorder_parts(sentence: &mut Sentence) -> &mut Sentence {
    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Reorder the words.");
        print_enumerated(&sentence.splits[sentence.current_split]);
        print_boxed("Move up: u    Move down: d    Accept: a     Revert: r");
        let entry = read_input();
        match entry.as_str() {
            "a" => return sentence,
            "r" => revert_splits(sentence),
            "u" => move_up(sentence, get_num_choice("Which word? ")),
            "d" => apply_reorder(sentence, get_num_choice("Which word? ")),
            _ => continue,
        }
    }
}

fn move_up(sentence: &mut Sentence, idx: usize) {
    if idx == 0 {
        println!("You cannot move the first one earlier.");
        return;
    }
    if idx > sentence.splits[sentence.current_split].len() {
        println!("That number is too high! Try again!");
        return;
    }
    let mut new_split: Vec<String> = Vec::new();
    let prev_split = &sentence.splits[sentence.current_split];
    let max = prev_split.len();
    for word in prev_split[..idx - 1].iter() {
        new_split.push(word.clone());
    }
    new_split.push(prev_split[idx].clone());
    new_split.push(prev_split[idx - 1].clone());

    if idx + 1 < max {
        for word in prev_split[idx + 1..].iter() {
            new_split.push(word.clone());
        }
    }
    sentence.splits.push(new_split);
    sentence.current_split += 1;
}

fn apply_reorder(sentence: &mut Sentence, idx: usize) {
    if idx + 2 > sentence.splits[sentence.current_split].len() {
        println!("You cannot join the last word to a 'next word'. There is no 'next word'!");
        return;
    }
    let mut new_split: Vec<String> = Vec::new();
    let prev_split = &sentence.splits[sentence.current_split];
    let max = prev_split.len();
    for word in prev_split[..idx].iter() {
        new_split.push(word.clone());
    }
    new_split.push(prev_split[idx + 1].clone());
    new_split.push(prev_split[idx].clone());

    if idx + 2 < max {
        for word in prev_split[idx + 2..].iter() {
            new_split.push(word.clone());
        }
    }
    sentence.splits.push(new_split);
    sentence.current_split += 1;
}

fn set_answers(sentence: &mut Sentence) {
    let max_ans_vecs = sentence.splits[sentence.current_split].len();
    for _ in 0..max_ans_vecs {
        let new_vec: Vec<AnswerOption> = Vec::new();
        sentence.answers.push(new_vec);
    }

    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Enter some answers.");
        print_enumerated_answers(&sentence);
        print_boxed("Edit answers: e    Complete: c    Return to menu: m");
        let entry = read_input();
        match entry.as_str() {
            "c" => {
                // sentence.completed = true;
                if check_for_complete(sentence) {
                    break;
                } else {
                    continue;
                }
            }
            "m" => {
                break;
            }
            "e" => {
                let idx = get_num_choice("Which no.? ");
                add_answer_dialog(sentence, idx);
            }
            _ => continue,
        }
    }
}

fn check_for_complete(sentence: &mut Sentence) -> bool {
    if sentence.answers[0].len() == 0 {
        print_boxed(
            "You cannot mark this complete:\n\
                     you haven't entered any answers.\n\
                     Continue: c",
        );
        read_input();
        return false;
    } else {
        sentence.completed = true;
        return true;
    }
}

fn add_answer_dialog(sentence: &mut Sentence, idx: usize) {
    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Enter/edit answers.");
        print_answer_options(sentence, idx);
        print_boxed("Add: a    Edit: e    Delete: d    Accept: RET");
        match read_input().as_str() {
            "a" => add_answer(sentence, idx),
            "e" => edit_answer_dialog(sentence, idx),
            "d" => delete_answer_dialog(sentence, idx),
            "" => break,
            _ => break,
        }
    }
}

fn add_answer(sentence: &mut Sentence, idx: usize) {
    print_boxed("Enter an answer.");
    // Get the answer
    let answer = read_input();
    // Get the mark
    print_boxed("Choose a mark.");
    println!("1. 0%");
    println!("2. 100%");
    println!("3. 75%");
    println!("4. 66%");
    println!("5. 50%");
    println!("6. 33%");
    println!("7. 25%");
    print_line();

    let mark = match read_input().as_str() {
        "1" => 0,
        "2" => 100,
        "3" => 75,
        "4" => 66,
        "5" => 50,
        "6" => 33,
        "7" => 25,
        _ => 0,
    };
    // Get feedback
    let mut flag: bool = true;
    print_boxed("Choose the feedback.");
    println!("1. Try again!");
    println!("2. Well done!");
    println!("3. Look at your notes on nouns.");
    println!("4. Look at your notes on verbs.");
    println!("5. Look at your notes on adjectives.");
    println!("6. ###Not a question###");
    println!("7. Input something else.");
    print_line();
    let feedback = match read_input().as_str() {
        "1" => "Try again!".to_string(),
        "2" => "Well done!".to_string(),
        "3" => "Look at your notes on nouns.".to_string(),
        "4" => "Look at your notes on verbs.".to_string(),
        "5" => "Look at your notes on adjectives".to_string(),
        "6" => {
            flag = false;
            "###Not a question###".to_string()
        }
        "7" => {
            print!("Enter your feedback: ");
            format!("{}", read_input())
        }
        _ => "Try again!".to_string(),
    };
    let answeroption = AnswerOption {
        is_question: flag,
        mark: mark,
        answer: answer,
        feedback: feedback,
    };
    sentence.answers[idx].push(answeroption);
}

fn edit_answer_dialog(sentence: &mut Sentence, idx: usize) {
    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Choose an answer.");
        print_answer_options(sentence, idx);
        print_boxed("Edit: num    Mark non-question: m    Delete: d    Accept: a");
        let entry = read_input();
        match entry.as_str() {
            "d" => delete_answer_dialog(sentence, idx),
            "a" => break,
            "m" => mark_non_question(sentence, idx, get_num_choice("Which no.? ")),
            "" => continue,
            _ => {
                // let opt = entry.trim().parse::<usize>().unwrap();
                // edit_answer(sentence, idx, opt - 1);
                edit_answer(sentence, idx, get_num_choice("Which no.? "));
            }
        }
    }
}

fn mark_non_question(sentence: &mut Sentence, idx: usize, opt: usize) {
    let answer_struct = &mut sentence.answers[idx][opt];
    answer_struct.mark = 0;
    answer_struct.feedback = "###Not a question###".to_string();
    answer_struct.is_question = false;
}

fn delete_answer_dialog(sentence: &mut Sentence, idx: usize) {
    loop {
        // clear the screen
        print!("\x1B[2J");
        print_boxed("Delete an answer.");
        print_answer_options(sentence, idx);
        print_boxed("Delete: num    Accept: RET");
        let entry = read_input();
        match entry.as_str() {
            "" => break,
            _ => {
                let opt = entry.trim().parse::<usize>().unwrap();
                delete_answer(sentence, idx, opt - 1);
            }
        }
    }
}

fn edit_answer(sentence: &mut Sentence, idx: usize, opt: usize) {
    let answer_struct = &mut sentence.answers[idx][opt];
    let previous_mark = &mut answer_struct.mark;
    let previous_answer = &mut answer_struct.answer;
    let previous_feedback = &mut answer_struct.feedback;
    answer_struct.is_question = true;

    print_boxed("Edit an answer.");
    // Get the answer
    println!("Answer: {}", previous_answer);
    let mut rl = rustyline::Editor::<()>::new();
    let readline = rl.readline_with_initial(">> ", (&previous_answer[..], ""));
    match readline {
        Ok(line) => *previous_answer = line,
        Err(_) => println!("No input"),
    }

    // Get the mark
    print_boxed("Choose a mark.");
    println!("1. 0%");
    println!("2. 100%");
    println!("3. 75%");
    println!("4. 66%");
    println!("5. 50%");
    println!("6. 33%");
    println!("7. 25%");
    print_line();

    println!("Current mark: {}", previous_mark);
    match read_input().as_str() {
        "1" => *previous_mark = 0,
        "2" => *previous_mark = 100,
        "3" => *previous_mark = 75,
        "4" => *previous_mark = 66,
        "5" => *previous_mark = 50,
        "6" => *previous_mark = 33,
        "7" => *previous_mark = 25,
        _ => {}
    }

    // Get feedback
    print_boxed("Choose the feedback.");
    println!("1. Try again!");
    println!("2. Well done!");
    println!("3. Look at your notes on nouns.");
    println!("4. Look at your notes on verbs.");
    println!("5. Look at your notes on adjectives.");
    println!("6. Input something else.");
    print_line();
    println!("Current feedback: {}", previous_feedback);
    match read_input().as_str() {
        "1" => *previous_feedback = "Try again!".to_string(),
        "2" => *previous_feedback = "Well done!".to_string(),
        "3" => *previous_feedback = "Look at your notes on nouns.".to_string(),
        "4" => *previous_feedback = "Look at your notes on verbs.".to_string(),
        "5" => *previous_feedback = "Look at your notes on adjectives".to_string(),
        "6" => {
            print!("Enter your feedback: ");
            *previous_feedback = format!("{}", read_input());
        }
        _ => {}
    }
}

fn delete_answer(sentence: &mut Sentence, idx: usize, opt: usize) {
    sentence.answers[idx].remove(opt);
}
