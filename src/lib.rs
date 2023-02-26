use std::io::Result;

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
pub enum ScaleType {
    Major(Key),
    Minor(Key),
}
impl ScaleType {
    fn select_scale() -> Self {
        let mut rng = rand::thread_rng();
        let rng_n: i32 = rng.gen();
        match &rng_n {
            &rng_n if &rng_n % 2 == 0 => Self::Major(Key::select_key()),
            _ => Self::Minor(Key::select_key()),
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
#[derive(Debug)]
struct Scale<'a> {
    // scale_type:ScaleType,
    notes: Vec<&'a str>,
}

impl Scale<'_> {
    fn new(vsc_type: ScaleType) -> Self {
        Self {
            // scale_type:vsc_type,
            notes: match vsc_type {
                ScaleType::Major(Key::C) => vec!["C", "D", "E", "F", "G", "A", "B"],
                ScaleType::Major(Key::G) => vec!["G", "A", "B", "C", "D", "E", "Gb"],
                ScaleType::Major(Key::D) => vec!["D", "E", "Gb", "G", "A", "B", "Db"],
                ScaleType::Major(Key::A) => vec!["A", "B", "Db", "D", "E", "Gb", "Ab"],
                ScaleType::Major(Key::E) => vec!["E", "GB", "Ab", "A", "B", "Db", "Ed"],
                ScaleType::Major(Key::B) => vec!["B", "Db", "Eb", "E", "Gb", "Ab", "Bb"],
                ScaleType::Major(Key::Gb) => vec!["Gb", "Ab", "Bb", "B", "Db", "Eb", "F"],
                ScaleType::Major(Key::Db) => vec!["Db", "Eb", "F", "Gb", "Ab", "Bb", "C"],
                ScaleType::Major(Key::Ab) => vec!["Ab", "Bb", "C", "Db", "Eb", "F", "G"],
                ScaleType::Major(Key::Eb) => vec!["Eb", "F", "G", "Ab", "Bb", "C", "D"],
                ScaleType::Major(Key::Bb) => vec!["Bb", "C", "D", "Eb", "F", "G", "A"],
                ScaleType::Major(Key::F) => vec!["F", "G", "A", "Bb", "C", "D", "E"],
                ScaleType::Minor(Key::C) => vec!["C", "D", "Eb", "F", "G", "Ab", "Bb"],
                ScaleType::Minor(Key::G) => vec!["G", "A", "Bb", "C", "D", "Eb", "F"],
                ScaleType::Minor(Key::D) => vec!["D", "E", "F", "G", "A", "Bb", "C", "D"],
                ScaleType::Minor(Key::A) => vec!["A", "B", "C", "D", "E", "F", "G"],
                ScaleType::Minor(Key::E) => vec!["E", "Gb", "G", "A", "B", "C", "D"],
                ScaleType::Minor(Key::B) => vec!["B", "Db", "D", "E", "Gb", "G", "A"],
                ScaleType::Minor(Key::Gb) => vec!["Gb", "Ab", "A", "B", "Db", "D", "E"],
                ScaleType::Minor(Key::Db) => vec!["Db", "Eb", "E", "Gb", "Ab", "A", "B"],
                ScaleType::Minor(Key::Ab) => vec!["Ab", "Bb", "B", "Db", "Eb", "E", "Gb"],
                ScaleType::Minor(Key::Eb) => vec!["Eb", "F", "Gb", "Ab", "Bb", "Cb", "Dd"],
                ScaleType::Minor(Key::Bb) => vec!["Bb", "C", "Db", "Eb", "F", "Gb", "Ab"],
                ScaleType::Minor(Key::F) => vec!["F", "G", "Ab", "Bb", "C", "Db", "Eb"],
            },
        }
    }
    fn scale_shuffle(&mut self) {
        let mut rng = StepRng::new(1, 13);
        let mut irs = Irs::default();
        irs.shuffle(&mut self.notes, &mut rng).unwrap();
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
        todo!()
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
}
pub fn init() -> requestty::Result<()> {
    // todo!();
    let q = Question::input("test").message("Give me an imput").build();
    let _a = requestty::prompt_one(q)?;
    // eprintln!("{:?}",a);
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
