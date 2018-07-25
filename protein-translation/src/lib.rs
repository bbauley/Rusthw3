pub struct vector_func {
  vector: Vec<(&'static str, &'static str)>,  //Modify this into a hashmap for efficiency
}

impl vector_func {
  pub fn name_for(&self, codon: &str) -> Result<&str, &str> {
    for tuple in &self.vector {
      if tuple.0 == codon {
        return Ok(tuple.1);
      }
    }
    Err("NAME NOT FOUND IN VECTOR")
  }
}

pub fn parse(v: Vec<(&'static str, &'static str)>) -> vector_func {

  vector_func {vector: v}
}
