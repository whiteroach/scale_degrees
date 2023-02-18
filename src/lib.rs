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
// pub enum ScaleType<'a> {
//     Major(Box<& 'a str>),
//     Minor(Box<& 'a str>)
// }
// impl Distribution<Spinner> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Spinner {
//         // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
//         match rng.gen_range(0..=2) { // rand 0.8
//             0 => Spinner::One,
//             1 => Spinner::Two,
//             _ => Spinner::Three,
//         }
//     }
// }
//
// fn main() {
//     let spinner: Spinner = rand::random();
//     println!("{:?}", spinner);
// }
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
struct Scale<'a> {
    scale_type:ScaleType,
    notes:Vec<& 'a str>
}

impl Scale<'_> {
   fn new(&self,key:Key, vsc_type:ScaleType)->Self{
        Self {
            scale_type:vsc_type,
            notes:match self.scale_type {
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

fn select_scale()->Result<ScaleType> {
    let mut rng = rand::thread_rng(); 
    let rng_n:i32= rng.gen();
    let key:Key = rand::random();
    match &rng_n {
        &rng_n if &rng_n % 2 == 0 => Ok(ScaleType::Major(key)),
        _=>    Ok(ScaleType::Minor(key)),
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
struct Exercise {}

pub fn init() -> requestty::Result<()> {
    // todo!();
    let q = Question::input("test").message("Give me an imput").build();
    let a = requestty::prompt_one(q)?;
    // eprintln!("{:?}", Keys::Ab(vec!["A".to_string()]));

    eprintln!("{:?}",select_scale()) ;

    Ok(())
}
