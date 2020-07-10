use crate::read_input;
use crate::AnswerOption;
use crate::Sentence;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn print_enumerated(words: &Vec<String>) {
    for (num, word) in words.iter().enumerate() {
        let num = num + 1;
        println!("{}: {}", num, word);
    }
}

pub fn print_enumerated_answers(sentence: &Sentence) {
    for (num, word) in sentence.splits[sentence.current_split].iter().enumerate() {
        println!("{}: {}", num + 1, word);
        if num < sentence.answers.len() {
            let max_rows = sentence.answers[num].len();
            for i in 0..max_rows {
                let answer = &sentence.answers[num][i];
                println!(
                    "   |{:5} | {:35}| {:29}",
                    answer.mark, answer.answer, answer.feedback
                );
            }
        }
    }
}

pub fn print_answer_options(sentence: &mut Sentence, idx: usize) {
    print_boxed(
        format!(
            "Question word: {}",
            sentence.splits[sentence.current_split][idx]
        )
        .as_str(),
    );
    if sentence.answers[idx].len() == 0 {
        println!("");
        println!("Enter your first answer.");
        println!("");
    } else {
        // let max_rows = sentence.answers[idx].len();
        println!(
            "    +------+------------------------------------+------------------------------+"
        );
        println!(
            "    | Mark | Answer                             | Feedback                     |"
        );
        println!(
            "+---+------+------------------------------------+------------------------------+"
        );
        for (num, word) in sentence.answers[idx].iter().enumerate() {
            // println!("{}. {:?}", num + 1, word);
            println!(
                "|{:2}.|{:5} | {:35}| {:29}|",
                num + 1,
                word.mark,
                word.answer,
                word.feedback
            );
        }
        println!(
            "+---+------+------------------------------------+------------------------------+"
        );
        println!("");
    }
}

pub fn print_boxed(content: &str) {
    println!("{}{}{}", "+", "-".repeat(78), "+");
    for line in content.split("\n") {
        println!("| {:76} |", line);
    }
    println!("{}{}{}", "+", "-".repeat(78), "+");
}

pub fn print_line() {
    println!("{}", "-".repeat(80));
}

pub fn print_questions(bank: &Vec<Sentence>) {
    println!("");
    if bank.len() == 0 {
        println!("Press a to enter your first question...");
    } else {
        for (num, question) in bank.iter().enumerate() {
            let status: &str = match question.completed {
                true => "D",
                false => "i",
            };
            println!(" {} : {}. {}", status, num + 1, question.initial_sentence);
        }
    }
    println!("");
}

pub fn move_print_questions(bank: &mut Vec<Sentence>, idx: usize) {
    let mut flag: &str;
    for (num, question) in bank.iter().enumerate() {
        if num == idx {
            flag = ">";
        } else {
            flag = " ";
        }
        println!(" {} | {}", flag, question.initial_sentence);
    }
}

pub fn print_to_file(bank: &Vec<Sentence>) {
    // Put opening statement in xml file
    // fields are stage_number, folder_name
    macro_rules! xml_start {
        ($arg1:expr, $arg2:expr) => {
            format!(
                "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
             <quiz>\n\
             <!-- question: 0  -->\n\
             <question type=\"category\">\n\
             <category>\n\
             <text>$course$/{}/Composition/{}</text>\n\
             </category>\n\
             </question>\n",
                $arg1, $arg2
            )
            .as_bytes()
        };
    }

    // fields are question_number, question_name, question, questioncode
    macro_rules! xml_question {
        ($q_num:expr, $q_name:expr, $question:expr, $q_code:expr) => {
            format!(
                "<!-- question: {}  -->\n\
                 <question type=\"cloze\" > \n\
                 <name>\n\
                 <text>{}</text>\n\
                 </name>\n\
                 <questiontext>\n\
                 <text>\n\
                 <![CDATA[<p>{}</p>\n\
                 <p><font size=\"4\" face=\"times new roman,times,serif\">{}.</font></p>]]>\n\
                 </text>\n\
                 </questiontext>\n\
                 <generalfeedback>\n\
                 <text></text>\n\
                 </generalfeedback>\n\
                 <shuffleanswers>0</shuffleanswers>\n\
                 </question>\n",
                $q_num, $q_name, $question, $q_code
            ).as_bytes()
        }
    }

    let xml_end: String = String::from("</quiz>\n");

    print!("Which stage is this for? ");
    let stage_number = read_input();
    print!("Please supply a exercise name. ");
    let ex_name = read_input().replace(" ", "_");
    print!("Please enter a question title base for moodle to use internally. ");
    // let question_title = read_input();
    let question_title = format!("{}_q", ex_name);

    // set up Writer
    let f = File::create("./upload.xml").expect("Unable to create file");
    let mut writer = BufWriter::new(f);

    // make initial write
    writer
        .write(xml_start!(stage_number, ex_name))
        .expect("Unable to write xml start.");

    let mut question_number: u32 = 1000;
    let mut question_name: String;
    let mut english: &str;
    let mut question_code = String::new();
    //Now loop over question data
    for question in bank.iter() {
        question_number += 1;
        question_name = format!("{}_{}", question_title, question_number);
        english = question.initial_sentence.as_str();
        // question_code = moodle_shortanswer(&question.answers[num]);
        question_code.clear();
        for group in question.answers.iter() {
            if group[0].is_question {
                question_code = format!("{} {}", question_code, moodle_shortanswer(&group));
            } else {
                question_code = format!("{} {}", question_code, group[0].answer)
            }
        }
        writer
            .write(xml_question!(
                question_number,
                question_name,
                english,
                question_code
            ))
            .expect("Unable to write xml question.");
    }
    writer
        .write(xml_end.as_bytes())
        .expect("Unable to write xml end.");
    writer.flush().expect("Unable to write data.");
}

fn moodle_shortanswer(answers: &Vec<AnswerOption>) -> String {
    let mut question_string = String::new();
    for answer in answers {
        question_string = format!(
            "{}~%{}%{}#{}",
            question_string, answer.mark, answer.answer, answer.feedback
        );
    }
    // format!("{{1:MULTICHOICE:{}}}", question_string)
    format!("{{1:SHORTANSWER:{}}}", question_string)
}
