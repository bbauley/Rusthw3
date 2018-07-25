pub fn encrypt(input: &str) -> String {
    let mut string = "".to_string();
    for letter in input.chars() {   //removing all punctuation, spaces, and uppercase letters
        if letter.is_alphabetic() {
            string.push_str(&letter.to_lowercase().to_string()); //need to convert lowercase iterator to &str
        }
    }
    let length = string.len();
    let mut result = "".to_string();
    if length != 0 {
        let rect_size = compute_rect(length);   //rect_size.0(num of rows) rect_size.1(num of cols)
        let mut rect:Vec<Vec<char>> = Vec::with_capacity(rect_size.1); 
        for _i in 0..rect_size.1 {                           //allocating 2d vector space
            rect.push(Vec::with_capacity(rect_size.0));  
        }
        let mut j = 0;
        for (i, letter) in string.as_str().chars().enumerate() {    
            let i_mod = i % rect_size.1;
            if i != 0 && i_mod == 0 {
                j = j + 1;
            } 
            rect[j].push(letter);   //pushing all characters from string into 2d vector
        }
        for col in 0..rect_size.1 {
            for row in 0..rect_size.0 {
                result.push(rect[row][col]);    //Pushing all elements in 2d vector into resulting String
            }
            if col != rect_size.1 - 1 { //Don't need to add an extra space at the very end of the string
                result.push_str(" ");
            }
        } 
    }
    result
}

fn compute_rect(length:usize) -> (usize, usize) {
    let mut r = 1;
    let mut c = 1;
    while (r * c) < length {
        if c == r {
            c = c + 1;
        }
        else {
            r = r + 1; 
        }
    }
    (r, c)
}
