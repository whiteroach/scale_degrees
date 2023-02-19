use std::io::Result;

use rand::{thread_rng,Rng,rngs::mock::StepRng, distributions::{Distribution, Standard}};
use rand::seq::SliceRandom;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use requestty::{Answer, Question};

 
#[derive(Debug)]
pub enum ScaleType{
    Major(Key),
    Minor(Key)
} 
impl ScaleType {
    fn select_scale() -> Self {
        let mut rng = rand::thread_rng(); 
        let rng_n:i32= rng.gen();
        match &rng_n {
            &rng_n if &rng_n % 2 == 0 => Self::Major(Key::select_key()),
            _=> Self::Minor(Key::select_key()),
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
    F
}
impl Key {
    fn new(key_name:&str) -> Option<Self> {
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
            _ => None

        }
    }
    fn select_key() -> Self {
        let key:Self = rand::random();
        key
    }
    fn compare(&self, answer:&str) -> bool {
        // answer.to_ascii_uppercase()
        match self {
            &Self::C if answer == "C" => true,
            &Self::D if answer == "D" => true,
            &Self::A if answer == "A" => true,
            &Self::E if answer == "E" => true,
            &Self::B if answer == "B" => true,
            &Self::Gb if answer == "Gb" => true,
            &Self::Db if answer == "Db" => true,
            &Self::Ab if answer == "Ab" => true,
            &Self::Eb if answer == "Eb" => true,
            &Self::Bb if answer == "Bb" => true,
            &Self::F if answer == "F" => true,
            _ => false

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
            _ => Key::F
        }
    }
}
#[derive(Debug)]
struct Scale<'a> {
    // scale_type:ScaleType,
    notes:Vec<& 'a str>
}

impl Scale<'_> {
   fn new( vsc_type:ScaleType)->Self{
        Self {
            // scale_type:vsc_type,
            notes:match vsc_type {
                ScaleType::Major(Key::C) => vec!["C","D","E","F","G","A","B"], 
                ScaleType::Major(Key::G) => vec!["G","A","B","C","D","E","Gb"],
                ScaleType::Major(Key::D) => vec!["D","E","Gb","G","A","B","Db"],               
                ScaleType::Major(Key::A) => vec!["A","B","Db","D","E","Gb","Ab"],
                ScaleType::Major(Key::E) => vec!["E","GB","Ab","A","B","Db","Ed"],   
                ScaleType::Major(Key::B) => vec!["B","Db","Eb","E","Gb","Ab","Bb"],
                ScaleType::Major(Key::Gb) => vec!["Gb","Ab","Bb","B","Db","Eb","F"],
                ScaleType::Major(Key::Db) => vec!["Db","Eb","F","Gb","Ab","Bb","C"],
                ScaleType::Major(Key::Ab) => vec!["Ab","Bb","C","Db","Eb","F", "G"],
                ScaleType::Major(Key::Eb) => vec!["Eb","F","G","Ab","Bb","C","D"],
                ScaleType::Major(Key::Bb) => vec!["Bb","C","D","Eb","F","G","A"],
                ScaleType::Major(Key::F) => vec!["F","G","A","Bb","C","D","E"],
                ScaleType::Minor(Key::C) => vec!["C","D","Eb","F","G","Ab","Bb"], 
                ScaleType::Minor(Key::G) => vec!["G","A","Bb","C","D","Eb","F"],
                ScaleType::Minor(Key::D) => vec!["D","E","F","G","A","Bb","C","D"],
                ScaleType::Minor(Key::A) => vec!["A","B","C","D","E","F","G"],
                ScaleType::Minor(Key::E) => vec!["E","Gb","G","A","B","C","D"],   
                ScaleType::Minor(Key::B) => vec!["B","Db","D","E","Gb","G","A"],
                ScaleType::Minor(Key::Gb) => vec!["Gb","Ab","A","B","Db","D","E"],
                ScaleType::Minor(Key::Db) => vec!["Db","Eb","E","Gb","Ab","A","B"],
                ScaleType::Minor(Key::Ab) => vec!["Ab","Bb","B","Db","Eb","E","Gb"],
                ScaleType::Minor(Key::Eb) => vec!["Eb","F","Gb","Ab","Bb","Cb","Dd"],
                ScaleType::Minor(Key::Bb) => vec!["Bb","C","Db","Eb","F","Gb","Ab"],
                ScaleType::Minor(Key::F) => vec!["F","G","Ab","Bb","C","Db","Eb"],
            }
        }
    } 
}

// use shuffle::shuffler::Shuffler;
// use shuffle::irs::Irs;
// use rand::rngs::mock::StepRng;
//
// let mut rng = StepRng::new(2, 13);
// let mut irs = Irs::default();
//
// let mut input = vec![1, 2, 3, 4, 5];
//
// irs.shuffle(&mut input, &mut rng);
//
// fn scale_shuffle() -> Result<String> {
//     todo!();
//     let mut rng = StepRng::new(2, 13);
//     let mut irs:Irs<Vec<String>> = Irs::default();
//     //need to check also if the scale is major or minor, using the enum.
//     match select_scale()? {
//         "Ab" => todo!(),
//         "A" => todo!(),
//         "Bb" => todo!(),
//         "C" => todo!(),
//         "Db" => todo!(),
//         "D" => todo!(),
//         "Eb" => todo!(),
//         "E" => todo!(),
//         "F" => todo!(),
//         "Gb" => todo!(),
//         "G" => todo!(),
//         &_ => todo!()
//     }
// }
pub enum ExerciseType {
    Type_A,
    Type_B,
    Type_C,
    Type_D,
}
impl Distribution<ExerciseType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExerciseType {
        match rng.gen_range(0..=3) {
            0 => ExerciseType::Type_A,
            1 => ExerciseType::Type_B,
            2 => ExerciseType::Type_C,
            _ => ExerciseType::Type_D,
        }
    }
}
impl ExerciseType {
    fn select_exercise_type() -> Self {
            let ex_type:Self = rand::random();
            ex_type
    }
}

struct Exercise<'a> {
    ex_type:ExerciseType,
    question:Question<'a>
}
pub fn init() -> requestty::Result<()> {
    // todo!();
    let q = Question::input("test").message("Give me an imput").build();
    let a = requestty::prompt_one(q)?;
    // eprintln!("{:?}",a);
    // eprintln!("{:?}",ScaleType::select_scale()) ;
    // eprintln!("{:?}",Scale::new(ScaleType::select_scale()));
    // let k = Key::select_key();
    // let kk = Key::new("C").unwrap();
    // eprintln!("{}",kk.compare("C"));
    Ok(())
}
