use crate::utils::is_ascii_alphanumeric;
use pinyin::ToPinyin;
use std::collections::HashSet;
use tantivy::tokenizer::Tokenizer;
use tantivy_jieba::JiebaTokenizer;

static TOKENIZER: JiebaTokenizer = tantivy_jieba::JiebaTokenizer {};
pub fn tokenize(hans: String) -> String {
  if is_ascii_alphanumeric(hans.as_str()) {
    return hans;
  }
  let mut token_stream = TOKENIZER.token_stream(&hans);

  let mut token_text: HashSet<String> = vec![].into_iter().collect();

  while let Some(token) = token_stream.next() {
    let raw = token.text.clone();
    let mut first = String::new();
    let mut all = String::new();
    token_text.insert(raw.clone());
    for pinyin in raw.as_str().to_pinyin() {
      if let Some(pinyin) = pinyin {
        first = format!("{}{}", first, pinyin.first_letter());
        all = format!("{}{}", all, pinyin.plain());
      }
    }
    if !first.is_empty() {
      token_text.insert(first);
    }
    if !all.is_empty() {
      token_text.insert(all);
    }
  }
  token_text.insert(hans.clone());
  token_text.into_iter().collect::<Vec<String>>().join(" ")
}

pub fn search_tokenize(hans: String) -> String {
  if is_ascii_alphanumeric(hans.as_str()) {
    return hans.as_str().to_lowercase();
  }
  let space = " ";
  let hans = hans.replace("-", space).replace("_", space);
  let mut token_stream = TOKENIZER.token_stream(&hans);

  let mut token_text: HashSet<String> = vec![].into_iter().collect();

  while let Some(token) = token_stream.next() {
    let raw = token.text.clone();
    token_text.insert(raw.clone());
  }
  token_text.insert(hans.clone());
  token_text.into_iter().collect::<Vec<String>>().join(space)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn t1() {
    let hans = "迅雷下载";
    let vec = tokenize(hans.to_string());
    println!("{:?}", vec);
  }
}
