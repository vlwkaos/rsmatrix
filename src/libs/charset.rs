use std::ops::{Range, RangeInclusive};
use rand::Rng;

pub enum CharWidth {
  Half = 1,
  Full = 2
}

#[derive(Debug, Clone)]
pub enum Charset {
  AlphaLowercase,
  AlphaUppercase,
  NumSym, 
  AlphaNumSym,
  Katakana, // japanese
  Emoji
}

impl Charset {
  pub fn get_charset_range(&self) -> Box<[RangeInclusive<char>]> {
    // non-primitive type return
    match *self {
      Charset::AlphaLowercase => ['\u{0061}'..='\u{007A}'].into(),
      Charset::AlphaUppercase => ['\u{0041}'..='\u{005A}'].into(),
      Charset::NumSym => ['\u{0021}'..='\u{0040}'].into(),
      Charset::AlphaNumSym => ['\u{0061}'..='\u{007A}',
                         '\u{0041}'..='\u{005A}',
                         '\u{0021}'..='\u{0040}'].into(),
      Charset::Katakana => ['\u{30A0}'..='\u{30FF}'].into(), 
      Charset::Emoji => ['\u{1F000}'..='\u{1F02F}', 
                         '\u{1F0A0}'..='\u{1F0FF}', 
                         '\u{1F680}'..='\u{1F6FF}',
                         '\u{1F910}'..='\u{1F96B}', 
                         '\u{1F980}'..='\u{1F9E0}'].into(),
    }
  }

  pub fn get_width(&self) -> u16 {
    let char_width = match *self {
      Charset::AlphaLowercase => CharWidth::Half,
      Charset::AlphaUppercase => CharWidth::Half,
      Charset::NumSym => CharWidth::Half,
      Charset::AlphaNumSym => CharWidth::Half,
      Charset::Katakana => CharWidth::Full, 
      Charset::Emoji => CharWidth::Full 
    };
    char_width as u16
  }

  pub fn get_random_char(&self) -> char {
    let mut rng = rand::thread_rng();
    let charset = self.get_charset_range();
    let idx = rng.gen_range(0..charset.len());
    // Range does not implement iterator over immutable reference..
    // cloning is cheap for range. just use it
    rng.gen_range(charset[idx].clone())
  }
}
