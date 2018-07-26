pub struct VectorFunc {
  vector: Vec<(&'static str, &'static str)>,  //Modify this into a hashmap for efficiency
}

impl VectorFunc {
  pub fn name_for(&self, codon: &str) -> Result<&str, &str> {
    for tuple in &self.vector {
      if tuple.0 == codon {
        return Ok(tuple.1);
      }
    }
    Err("NAME NOT FOUND IN VECTOR")
  }
  pub fn of_rna(&self, rna: &str) -> Result<Vec<&str>, &str> {
    let length = rna.len();
    if length % 3 != 0 {
      return Err("INVALID STRING LENGTH");
    }

    let mut split_codons:Vec<String> = Vec::new();
    let mut initial = rna.to_string();
    let mut temp = initial.split_off(3);
    while initial.len() != 0 {
      let copy = initial.clone();
      split_codons.push(copy);
      initial = temp.clone();
      if initial.len() >= 3 {
        temp = initial.split_off(3);
      }
    }
    let mut result:Vec<&str> = Vec::new();
    for codon in split_codons {
      let codon_result = self.name_for(&codon);
      if codon == "UAA" || codon == "UAG" || codon == "UGA" {
        break;
      }
      match codon_result {
        Ok(codon_result) => {                       
            result.push(codon_result)
        }
        Err(codon_result) => return Err(codon_result)
      }
    }
    if result.len() > 1 {
      Ok(result)
    }
    else {
      Err("NOT A VALID SEQUENCE OF CHARACTERS. LOOK AT README FOR MORE INFORMATION")
    }
  }

 }

pub fn parse(v: Vec<(&'static str, &'static str)>) -> VectorFunc {
  VectorFunc {vector: v}
}
