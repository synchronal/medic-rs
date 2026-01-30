use serde::de::{self, SeqAccess, Visitor, value};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StringOrList(#[serde(deserialize_with = "string_or_vec")] pub Vec<String>);

impl IntoIterator for StringOrList {
  type Item = String;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl AsRef<[String]> for StringOrList {
  fn as_ref(&self) -> &[String] {
    self.0.as_slice()
  }
}

impl<'a> IntoIterator for &'a StringOrList {
  type Item = &'a String;
  type IntoIter = std::slice::Iter<'a, String>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
  D: Deserializer<'de>,
{
  struct StringOrVec;

  impl<'de> Visitor<'de> for StringOrVec {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("string or list of strings")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(vec![s.to_owned()])
    }

    fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
    where
      S: SeqAccess<'de>,
    {
      Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
    }
  }

  deserializer.deserialize_any(StringOrVec)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn iterator_yields_string_references_not_clones() {
    let string_or_list = StringOrList(vec!["first".to_string(), "second".to_string()]);
    let mut iter = (&string_or_list).into_iter();

    let first_item = iter.next().unwrap();
    let second_item = iter.next().unwrap();

    assert_eq!(
      first_item.as_str() as *const str,
      string_or_list.0[0].as_str() as *const str
    );
    assert_eq!(
      second_item.as_str() as *const str,
      string_or_list.0[1].as_str() as *const str
    );

    assert_eq!(first_item, "first");
    assert_eq!(second_item, "second");
  }
}
