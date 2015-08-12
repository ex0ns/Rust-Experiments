use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

fn main() {
    let x = "PLDRX PLDRX OHV FKDWV VRQW SOHLQV GH FDOLQV";
    let frequencies = language_frequency("french");
    println!("{}", break_caesar(x));
}

fn text_frequency(ctext: &str) -> HashMap<char, f32> {
    let mut frequencies : HashMap<char, f32> = HashMap::new();

    ctext.chars().fold(&mut frequencies, |m, ch| {
        let r = *m.get(&ch).unwrap_or(&0f32);
        m.insert(ch, 1.0 + r);
        m
    });
    
    for (_, freq) in frequencies.iter_mut() {
        *freq = *freq/ctext.len() as f32 * 100f32;
    }
    frequencies
}

fn map_frequencies(language: HashMap<char, f32>, encoded: HashMap<char, f32>) -> HashMap<char, char> {
   let mut mapping: HashMap<char, char> = HashMap::new();
   let mut l_vec: Vec<(char, f32)> = vec![];
   let mut e_vec: Vec<(char, f32)> = vec![];

   for (letter, frequency) in language.iter() {
        l_vec.push((*letter, *frequency));
   }
   for (letter, frequency) in encoded.iter() {
        e_vec.push((*letter, *frequency));
   }

   l_vec.sort_by(|&(_, b), &(_ ,d)| d.partial_cmp(&b).unwrap_or(Ordering::Equal));
   e_vec.sort_by(|&(_, b), &(_ ,d)| d.partial_cmp(&b).unwrap_or(Ordering::Equal));

   println!("{:?} \n {:?}", l_vec, e_vec);

   let zip = e_vec.iter().zip(l_vec.iter()).collect::<Vec<_>>();
   for (&(encoded, _), &(language, _)) in zip {
        mapping.insert(encoded, language);
   }
   mapping
}

fn break_poly_sub(ctext: &str, language_freq: HashMap<char, f32>) -> String {
    let text_freq = text_frequency(ctext);
    let translate = map_frequencies(language_freq, text_freq);
    ctext.chars().map(|c| {
        translate.get(&c)
            .map_or(c, |&trans_ch| trans_ch)
    }).collect::<String>()
}

fn break_rot(ctext: &str, rot: i8) -> String {
    let bytes = ctext.to_string().into_bytes()
        .iter()
        .map(|c| (*c as i8 +rot) as u8)
        .collect::<Vec<u8>>();
    String::from_utf8(bytes).unwrap()
}

fn break_rot_13(ctext: &str) -> String {
    break_rot(ctext, -13i8)
}

fn break_caesar(ctext: &str) -> String {
    break_rot(ctext, -3i8)
}

fn language_frequency(ctext: &str) -> HashMap<char, f32> {
    let f = BufReader::new(File::open(ctext.to_string() + ".frequency").unwrap());
    let mut frequencies : HashMap<char, f32> = HashMap::new();

    for (_i, line) in f.lines().enumerate() {
        let line = line.unwrap();
        let mut line = line.split(" ");
        let letter = line.next().unwrap().chars().next();
        let freq : f32 = line.next()
            .unwrap_or_else(|| "0.0")
            .trim().parse().unwrap();
        match letter {
            Some(l) =>  { frequencies.insert(l, freq); },
            None => {}
        };
    }
    frequencies
}
