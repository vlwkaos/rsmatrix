use std::ops::{Range, RangeInclusive};
use rand::Rng;

const ALPHA_L_RANGE: [RangeInclusive<char>;1] = ['\u{0061}'..='\u{007A}'];
const ALPHA_U_RANGE: [RangeInclusive<char>;1] = ['\u{0041}'..='\u{005A}'];
const NUMSYM_RANGE: [RangeInclusive<char>;1] = ['\u{0021}'..='\u{0040}'];
const KATAKANA_RANGE: [RangeInclusive<char>;1] = ['\u{30A0}'..='\u{30FF}'];   
const EMOJI_RANGE: [RangeInclusive<char>;5] = ['\u{1F000}'..='\u{1F02F}', 
                                           '\u{1F0A0}'..='\u{1F0FF}', 
                                           '\u{1F680}'..='\u{1F6FF}',
                                           '\u{1F910}'..='\u{1F96B}', 
                                           '\u{1F980}'..='\u{1F9E0}'];
#[derive(Debug, Clone)]
pub enum Charset {
  AlphaLowercase,
  AlphaUppercase,
  NumSym, 
  Katakana, // japanese
  Emoji
}

impl Charset {
  pub fn get_charset_range(&self) -> Vec<RangeInclusive<char>> {
    // non-primitive type return
    match *self {
      Charset::AlphaLowercase => ALPHA_L_RANGE.into(),
      Charset::AlphaUppercase => ALPHA_U_RANGE.into(),
      Charset::NumSym => NUMSYM_RANGE.into(),
      Charset::Katakana => KATAKANA_RANGE.into(), 
      Charset::Emoji => EMOJI_RANGE.into(),
    }
  }

  pub fn get_random_char(&self) -> char {
    let mut rng = rand::thread_rng();
    let charset = self.get_charset_range();
    let idx = rng.gen_range(0..charset.len());
    // gen_range는 reference로 할 수 없다..
    match charset.into_iter().nth(idx) {
      Some(char_range) => rng.gen_range(char_range),
      _ => ' '
    }
  }
}
