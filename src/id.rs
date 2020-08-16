use rand::seq::IteratorRandom;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

pub fn gen() -> String {
  fold_into_id(ALPHABET.chars().choose_multiple(&mut rand::thread_rng(), 9))
}

fn fold_into_id(chars: Vec<char>) -> String {
  chars
    .chunks(3)
    .map(|chunk| chunk.iter().collect::<String>())
    .fold("xibt_gen".to_string(), |acc, chunk| acc + "-" + &chunk)
}

#[test]
fn id_folding() {
  assert_eq!(
    fold_into_id("abcdefghi".chars().collect()),
    "xibt_gen-abc-def-ghi"
  )
}
