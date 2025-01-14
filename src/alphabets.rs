use std::collections::HashMap;

const ALPHABETS: [(&'static str, &'static str); 22] = [
  ("numeric", "1234567890"),
  ("abcd", "abcd"),
  ("qwerty", "asdfqwerzxcvjklmiuopghtybn"),
  ("qwerty-homerow", "asdfjklgh"),
  ("qwerty-left-hand", "asdfqwerzcxv"),
  ("qwerty-right-hand", "jkluiopmyhn"),
  ("azerty", "qsdfazerwxcvjklmuiopghtybn"),
  ("azerty-homerow", "qsdfjkmgh"),
  ("azerty-left-hand", "qsdfazerwxcv"),
  ("azerty-right-hand", "jklmuiophyn"),
  ("qwertz", "asdfqweryxcvjkluiopmghtzbn"),
  ("qwertz-homerow", "asdfghjkl"),
  ("qwertz-left-hand", "asdfqweryxcv"),
  ("qwertz-right-hand", "jkluiopmhzn"),
  ("dvorak", "aoeuqjkxpyhtnsgcrlmwvzfidb"),
  ("dvorak-homerow", "aoeuhtnsid"),
  ("dvorak-left-hand", "aoeupqjkyix"),
  ("dvorak-right-hand", "htnsgcrlmwvz"),
  ("colemak", "arstqwfpzxcvneioluymdhgjbk"),
  ("colemak-homerow", "arstneiodh"),
  ("colemak-left-hand", "arstqwfpzxcv"),
  ("colemak-right-hand", "neioluymjhk"),
];

pub struct Alphabet<'a> {
  letters: &'a str,
}

impl<'a> Alphabet<'a> {
  fn new(letters: &'a str) -> Alphabet {
    Alphabet { letters }
  }

  pub fn hints(&self, matches: usize) -> Vec<String> {
    let letters: Vec<String> = self.letters.chars().map(|s| s.to_string()).collect();

    if letters.len() <= 1 {
      return letters;
    }

    let mut expansion: Vec<String> = letters.clone();
    let mut expand_idx = expansion.len() - 1;

    loop {
      if expansion.len() >= matches {
        break;
      }

      let prefix = expansion.remove(expand_idx);

      let sub_expansion: Vec<String> = letters
        .iter()
        .take(matches - expansion.len())
        .map(|s| prefix.clone() + s)
        .collect();

      expansion.splice(expand_idx..expand_idx, sub_expansion.clone());

      if expand_idx == 0 {
        expand_idx = expansion.len() - 1;
      } else {
        expand_idx -= 1;
      }
    }

    expansion.iter().take(matches).cloned().collect()
  }
}

pub fn get_alphabet(alphabet_name: &str) -> Alphabet {
  let alphabets: HashMap<&str, &str> = ALPHABETS.iter().cloned().collect();

  alphabets
    .get(alphabet_name)
    .expect(format!("Unknown alphabet: {}", alphabet_name).as_str()); // FIXME

  Alphabet::new(alphabets[alphabet_name])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_matches() {
    let alphabet = Alphabet::new("abcd");
    let hints = alphabet.hints(3);
    assert_eq!(hints, ["a", "b", "c"]);
  }

  #[test]
  fn composed_matches() {
    let alphabet = Alphabet::new("abcd");
    let hints = alphabet.hints(6);
    assert_eq!(hints, ["a", "b", "c", "da", "db", "dc"]);
  }

  #[test]
  fn composed_matches_two_arity() {
    let alphabet = Alphabet::new("abcd");
    let hints = alphabet.hints(8);
    assert_eq!(hints, ["a", "b", "ca", "cb", "da", "db", "dc", "dd"]);
  }

  #[test]
  fn composed_matches_three_arity() {
    let alphabet = Alphabet::new("ab");
    let hints = alphabet.hints(7);
    assert_eq!(hints, ["aa", "aba", "abb", "baa", "bab", "bba", "bbb"]);
  }
}
