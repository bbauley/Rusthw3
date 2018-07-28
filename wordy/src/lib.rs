pub struct WordProblem {
  formula: Vec<String>
}

/* This struct is here to clean up code in the 'answer' function
 * Look at 'answer' function documentation for more information
*/
struct State {
  lhs: isize,
  op: String,
  rhs: isize
}

impl WordProblem {
  /* Convert the str variable from the parameter into a vector
   *  while also removing any unecessary words that are not 
   *  relavent to the formula
   * eg. 
   * "What is 1 plus 1?" => ["1", "plus", "1"]
   * "What is 7 multiplied by 4 plus -2?" => ["7", "multiplied", "4", "plus", "-2"]
   * Storing the resulting vector into a struct and returning that struct
  */
  pub fn new(question: &str) -> WordProblem {
      let mut split:Vec<&str> = question.split(' ').collect();
      let mut form: Vec<String> = Vec::new();
      split.remove(0);
      split.remove(0);
      let length = split.len();
      split[length - 1] = split[length - 1].trim_matches('?');
      for i in 0..length {
        if split[i] != "by" {
          form.push(split[i].to_string());
        }
      }
      WordProblem {formula: form}
    }
  /* Loops through each element in the vector that was returned from the 'new' function
   * Updating elements in state struct as we traverse through vector
   * eg.
   * Looping through ["1", "plus", "1"]....
   *  lhs = 1 op = "" rhs = 0
   *  lhs = 1 op = "plus" rhs = 0
   *  lhs = 1 op = "plus" rhs = 1 -> lhs = 2 op = "" rhs = 0
   * Returns Ok(lhs) if successful or Err(message) if failed
  */
  pub fn answer(&self) -> Result<isize, String> {
    assert!(self.formula.len() != 0);
    let mut state = State{lhs: self.formula[0].parse::<isize>().unwrap_or(0), op: "".to_string(), rhs: 0};
    let length = self.formula.len();
    for i in 1..length {
      if i % 2 == 0 {
        let num = self.formula[i].parse::<isize>();
        match num {
          Ok(val) => {
            state.rhs = val;
            if state.op == "plus" {
              state.lhs = state.lhs + state.rhs;
            }
            else if state.op == "minus" {
              state.lhs = state.lhs - state.rhs;

            }
            else if state.op == "multiplied" {
              state.lhs = state.lhs * state.rhs;

            }
            else {
              state.lhs = state.lhs / state.rhs;
            }
            state.op = "".to_string();
            state.rhs = 0;
          }
          Err(_why) => return Err("Value not an integer where expected".to_string())
        }
      }
      else {
        let operator = &self.formula[i];
        match operator.as_str() {
          "plus" | 
          "minus" | 
          "multiplied" | 
          "divided" => state.op = operator.to_string(),
          _ => return Err("Operators need to be plus, minus, multiplied, or divided".to_string())
        }
      }
    }
    Ok(state.lhs)
  }
}

