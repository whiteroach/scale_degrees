use std::vec;
use std::fmt;

use rand::seq::SliceRandom;
use rand::{
    distributions::{Distribution, Standard},
    rngs::mock::StepRng,
 Rng,
};
use requestty::{Answer, Question };
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

#[derive(Debug)]
pub enum Scale {
    Major(Key),
    Minor(Key),
} 
impl Scale {
     fn select_scale() -> Self {
        let mut rng = rand::thread_rng();
        let rng_n: i32 = rng.gen();
        match &rng_n {
            &rng_n if &rng_n % 2 == 0 => Self::Major(Key::select_key()),
            _ => Self::Minor(Key::select_key()),
        }
    }
    // fn get_notes(&self) -> Vec<& 'static str > {
    fn get_notes(&self) -> Vec<&str > {
        match self {
                Scale::Major(Key::C) => vec!["C", "D", "E", "F", "G", "A", "B"],
                Scale::Major(Key::G) => vec!["G", "A", "B", "C", "D", "E", "Gb"],
                Scale::Major(Key::D) => vec!["D", "E", "Gb", "G", "A", "B", "Db"],
                Scale::Major(Key::A) => vec!["A", "B", "Db", "D", "E", "Gb", "Ab"],
                Scale::Major(Key::E) => vec!["E", "GB", "Ab", "A", "B", "Db", "Ed"],
                Scale::Major(Key::B) => vec!["B", "Db", "Eb", "E", "Gb", "Ab", "Bb"],
                Scale::Major(Key::Gb) => vec!["Gb", "Ab", "Bb", "B", "Db", "Eb", "F"],
                Scale::Major(Key::Db) => vec!["Db", "Eb", "F", "Gb", "Ab", "Bb", "C"],
                Scale::Major(Key::Ab) => vec!["Ab", "Bb", "C", "Db", "Eb", "F", "G"],
                Scale::Major(Key::Eb) => vec!["Eb", "F", "G", "Ab", "Bb", "C", "D"],
                Scale::Major(Key::Bb) => vec!["Bb", "C", "D", "Eb", "F", "G", "A"],
                Scale::Major(Key::F) => vec!["F", "G", "A", "Bb", "C", "D", "E"],
                Scale::Minor(Key::C) => vec!["C", "D", "Eb", "F", "G", "Ab", "Bb"],
                Scale::Minor(Key::G) => vec!["G", "A", "Bb", "C", "D", "Eb", "F"],
                Scale::Minor(Key::D) => vec!["D", "E", "F", "G", "A", "Bb", "C", "D"],
                Scale::Minor(Key::A) => vec!["A", "B", "C", "D", "E", "F", "G"],
                Scale::Minor(Key::E) => vec!["E", "Gb", "G", "A", "B", "C", "D"],
                Scale::Minor(Key::B) => vec!["B", "Db", "D", "E", "Gb", "G", "A"],
                Scale::Minor(Key::Gb) => vec!["Gb", "Ab", "A", "B", "Db", "D", "E"],
                Scale::Minor(Key::Db) => vec!["Db", "Eb", "E", "Gb", "Ab", "A", "B"],
                Scale::Minor(Key::Ab) => vec!["Ab", "Bb", "B", "Db", "Eb", "E", "Gb"],
                Scale::Minor(Key::Eb) => vec!["Eb", "F", "Gb", "Ab", "Bb", "Cb", "Dd"],
                Scale::Minor(Key::Bb) => vec!["Bb", "C", "Db", "Eb", "F", "Gb", "Ab"],
                Scale::Minor(Key::F) => vec!["F", "G", "Ab", "Bb", "C", "Db", "Eb"],
        }
    } 

    fn get_shuffled_notes(&self) -> Vec<&str> {
        let mut rng = StepRng::new(1, 13);
        let mut irs = Irs::default();
        let mut notes = self.get_notes();
        irs.shuffle(&mut notes, &mut rng).unwrap();
        notes
    }
}
impl fmt::Display for Scale { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
                Scale::Major(Key::C) => write!(f,"Major C"),   
                Scale::Major(Key::G) => write!(f,"Major G"), 
                Scale::Major(Key::D) => write!(f,"Major D"),         
                Scale::Major(Key::A) => write!(f,"Major A"),          
                Scale::Major(Key::E) => write!(f,"Major E"),     
                Scale::Major(Key::B) => write!(f,"Major B"),   
                Scale::Major(Key::Gb) => write!(f,"Major Gb"),   
                Scale::Major(Key::Db) => write!(f,"Major Db"),   
                Scale::Major(Key::Ab) => write!(f,"Major Ab"),   
                Scale::Major(Key::Eb) => write!(f,"Major Eb"),   
                Scale::Major(Key::Bb) => write!(f,"Major Bb"),   
                Scale::Major(Key::F) => write!(f,"Major F"),   
                Scale::Minor(Key::C) => write!(f,"Minor C"),   
                Scale::Minor(Key::G) => write!(f,"Minor G"),   
                Scale::Minor(Key::D) => write!(f,"Minor D"),   
                Scale::Minor(Key::A) => write!(f,"Minor A"),   
                Scale::Minor(Key::E) => write!(f,"Minor E"),   
                Scale::Minor(Key::B) => write!(f,"Minor B"),   
                Scale::Minor(Key::Gb) => write!(f,"Minor Gb"),   
                Scale::Minor(Key::Db) => write!(f,"Minor Db"),   
                Scale::Minor(Key::Ab) => write!(f,"Minor Ab"),   
                Scale::Minor(Key::Eb) => write!(f,"Minor Eb"),   
                Scale::Minor(Key::Bb) => write!(f,"Minor Bb"),   
                Scale::Minor(Key::F) => write!(f,"Minor F"),   
         }
     }
} 

#[derive(Debug)]
pub enum Degree {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven
}
impl Degree {
     fn select_degree() -> Self {
        let degree: Self = rand::random();
        degree
    }
    fn to_index(&self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
            Self::Four => 3,
            Self::Five => 4,
            Self::Six => 5,
            Self::Seven => 6
        }
    }
}
impl Distribution<Degree> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Degree {
        match rng.gen_range(0..=6) {
            0 => Degree::One,
            2 => Degree::Two,
            4 => Degree::Three,
            5 => Degree::Four,
            7 => Degree::Five,
            9 => Degree::Six,
            _ => Degree::Seven,
        }
    }
}
impl fmt::Display for Degree { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
                Degree::One => write!(f,"1st"),   
                Degree::Two => write!(f,"2nd"),         
                Degree::Three => write!(f,"3rd"),     
                Degree::Four => write!(f,"4th"),   
                Degree::Five => write!(f,"5th"),   
                Degree::Six => write!(f,"6th"),   
                Degree::Seven => write!(f,"7th"),   
         }
     }
} 
#[derive(Debug)]
pub enum Key {
    C,
    G,
    D,
    A,
    E,
    B,
    Gb,
    Db,
    Ab,
    Eb,
    Bb,
    F,
}
impl Key  {
    fn new(key_name: &str) -> Option<Self> {
        match key_name {
            "C" => Some(Self::C),
            "D" => Some(Self::D),
            "A" => Some(Self::A),
            "E" => Some(Self::E),
            "B" => Some(Self::B),
            "Gb" => Some(Self::Gb),
            "Db" => Some(Self::Db),
            "Ab" => Some(Self::Ab),
            "Eb" => Some(Self::Eb),
            "Bb" => Some(Self::Bb),
            "F" => Some(Self::F),
            _ => None,
        }
    }
    fn select_key() -> Self {
        let key: Self = rand::random();
        key
    }
    fn compare(&self, answer: &str) -> bool {
        // answer.to_ascii_uppercase()
        match *self {
            Self::C if answer == "C" => true,
            Self::D if answer == "D" => true,
            Self::A if answer == "A" => true,
            Self::E if answer == "E" => true,
            Self::B if answer == "B" => true,
            Self::Gb if answer == "Gb" => true,
            Self::Db if answer == "Db" => true,
            Self::Ab if answer == "Ab" => true,
            Self::Eb if answer == "Eb" => true,
            Self::Bb if answer == "Bb" => true,
            Self::F if answer == "F" => true,
            _ => false,
        }
    }
}
impl Distribution<Key> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Key {
        match rng.gen_range(0..=11) {
            0 => Key::C,
            1 => Key::G,
            2 => Key::D,
            3 => Key::A,
            4 => Key::E,
            5 => Key::B,
            6 => Key::Gb,
            7 => Key::Db,
            8 => Key::Ab,
            9 => Key::Eb,
            10 => Key::Bb,
            _ => Key::F,
        }
    }
}
pub enum ExerciseType {
    A,
    B,
    C,
    D,
} 
impl Distribution<ExerciseType> for Standard {
     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExerciseType {
         match rng.gen_range(0..=3) {
            0 => ExerciseType::A,
            1 => ExerciseType::B,
            2 => ExerciseType::C,
            _ => ExerciseType::D,
        }
    }
}
impl ExerciseType {
    fn select_exercise_type() -> Self {
        let ex_type: Self = rand::random();
        ex_type
    }
}

struct Exercise<'a> {
    ex_type:ExerciseType,
    question: Question<'a>,
    solution: String,
}

impl Exercise<'_> {
    fn new(ex_type: ExerciseType) -> Self {
        match ex_type {
            ExerciseType::A => Self::generate_ex_a(),
            ExerciseType::B => Self::generate_ex_b(),
            ExerciseType::C => Self::generate_ex_c(),
            ExerciseType::D => Self::generate_ex_d(),
        }
    }
    //A. What are the notes of {Scale} scale?
    fn generate_ex_a() -> Self {
        let scale = Scale::select_scale();
        //prep to shufle the choices
        let mut rng = StepRng::new(1, 13);
        let mut irs = Irs::default();

        let scale_shuffled = scale.get_shuffled_notes().join(",");
        let mut scale_vec = vec![scale_shuffled.clone(),Scale::select_scale().get_shuffled_notes().join(","),Scale::select_scale().get_shuffled_notes().join(","),Scale::select_scale().get_shuffled_notes().join(",")];
        irs.shuffle(&mut scale_vec, &mut rng).unwrap();

        let q =  Question::raw_select("type_a")
             .message(format!("What are the notes of the {} scale?", &scale.to_string()))
             .choices(scale_vec)
             .build();

        Self {
            ex_type: ExerciseType::A,
            question:q,
            solution:scale_shuffled
        }
    }
    //B. What note is the {Nth} of {Scale}?
    //- select a Scale
    //- generate the scale wih notes
    //- pick a note at random 
    //- save the index used to pick the note at random
    //
    fn generate_ex_b() -> Self {
        // todo!();
        let scale = Scale::select_scale();
        let scale_notes = scale.get_notes();
        let degree = Degree::select_degree(); 
        // let index = rand::thread_rng().gen_range(0..=6);
        let index = degree.to_index();
        let r_note = scale_notes[index];
        let q = Question::input("type_b")
            .message(format!("What note is the {} of {}",&degree.to_string(), &scale.to_string()))
            .build();
        Self {
            ex_type: ExerciseType::B,
            question:q,
            solution:r_note.to_string()
        }
    }
    //C. What degree of {Note} in reference to {Key}?
    fn generate_ex_c() -> Self {
        todo!()
    }
    //D. {Note}is the {Degree} of what note?
    fn generate_ex_d() -> Self {
        todo!()
    }
    fn ex_a_shuffler(scale_vec:Vec<Scale>) -> Vec<String> {
         todo!();
        // scale_vec.iter().map(|i| i::scale_shuffle())
        // for i in scale_vec.iter() {
        //     i.scale_shuffle();
        //     // i.to_string() // need to implement to display to scale, but first need to understand
        //     // if make sense make an enum out of Scale
        // };
    }
} 
fn match_answer(a:Answer) -> String {
    match a {
        Answer::ListItem(a) => a.text, 
        Answer::String(a) => a,
        _ => "invalid".to_string() 
    }
}
fn validate_answer(a:Answer,solution:String) {
    let answer = match_answer(a);
    eprintln!("a:{} s:{}",&answer,&solution);
    if answer == solution {
        eprintln!("Well done!")
    } else {
        eprintln!("Nope!")
    }
}
pub fn init() -> requestty::Result<()> {
    // todo!();
    // let q = Question::input("test").message("Give me an imput").build();
    // let _a = requestty::prompt_one(q)?;
    let i = Exercise::new(ExerciseType::A); 
    let a = requestty::prompt_one(i.question)?;
    validate_answer(a, i.solution);
    let i_two = Exercise::new(ExerciseType::B);
    let b = requestty::prompt_one(i_two.question)?;
    validate_answer(b, i_two.solution);
    // eprintln!("{:?}",b);
    Ok(())
}
