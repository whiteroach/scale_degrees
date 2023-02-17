use std::io::Result;

use rand::{thread_rng,Rng,rngs::mock::StepRng};
use rand::seq::SliceRandom;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use requestty::{Answer, Question};

 
#[derive(Debug)]
pub enum ScaleType<'a> {
    Major(Box<& 'a str>),
    Minor(Box<& 'a str>)
} 
// #[derive(Debug)]
// enum Keys {
//     Ab(Vec<String>),
//     A(Vec<String>),
//     Bb,
//     B,
//     C,
//     Db,
//     D,
//     Eb,
//     E,
//     F,
//     Gb,
//     G
// } 


pub fn select_scale()->Result<ScaleType<'static>> {
    let keys = vec!["Ab","A","Bb","B","C","Db","D","Eb","E","F","Gb","G"];
    let mut rng = rand::thread_rng();
    let mut rng_n = rng.gen();
    match &rng_n {
        &rng_n if &rng_n % 2 == 0 => Ok(ScaleType::Major(Box::new(*keys.choose(&mut rng).unwrap()))),
        _=>    Ok(ScaleType::Minor(Box::new(*keys.choose(&mut rng).unwrap()))),
  }
    // *keys.choose(&mut rng).unwrap()
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
