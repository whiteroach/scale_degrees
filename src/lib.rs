use std::vec;
use std::{fmt, io::Result};

use rand::seq::SliceRandom;
use rand::{
    distributions::{Distribution, Standard},
    rngs::mock::StepRng,
    thread_rng, Rng,
};
use requestty::{Answer, Question};
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
    fn get_notes(&self) -> Vec<& 'static str > {
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
    fn get_shuffled_notes(&self) -> Vec<& 'static str> {
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
impl Key {
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
    //ex_type:ExerciseType,
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
        todo!();
        let scale = Scale::select_scale();
        // let scale_vec = vec![Scale::new(scale),Scale::new(ScaleType::select_scale()),Scale::new(ScaleType::select_scale()),Scale::new(ScaleType::select_scale())];
        // Question::select("")
            // .message(format!("What are the notes of the {} scale?", &scale.to_string()))
            // .choice(text)
    }
    //B. What note is the {Nth} of {Key}?
    fn generate_ex_b() -> Self {
        todo!()
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
pub fn init() -> requestty::Result<()> {
    // todo!();
    let q = Question::input("test").message("Give me an imput").build();
    let _a = requestty::prompt_one(q)?;
    // eprintln!("{:?}",a);
    let l = Scale::select_scale();
    eprintln!("{:?}", &l);
    eprintln!("{:?}", &l.get_shuffled_notes());
 eprintln!("{:?}", l);

    // eprintln!("{:?}",ScaleType::select_scale()) ;
    //let mut sc = Scale::new(ScaleType::select_scale());
    //eprintln!("SC = {:?}",&sc);
    //sc.scale_shuffle();
    //eprintln!("SC-schuffled = {:?}",sc);
    // eprintln!("{:?}",Scale::new(ScaleType::select_scale()));
    // let k = Key::select_key();
    // let kk = Key::new("C").unwrap();
    // eprintln!("{}",kk.compare("C"));
    Ok(())
}
