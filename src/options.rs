//! a custom structure for user input handling
pub struct Options<'o> {
    options: &'o [String],
}

impl<'o> Options<'o> {
    /// Initializes Options
    pub fn new(values: &'o [String]) -> Self {
        Self { options: values }
    }

    /// Evaluates the choice given, checks if choice is a number
    /// indicating an index, otherwise it will match the internal
    /// values within self.values
    pub fn evaluate(&'o self, choice: &str) -> Option<String> {
        match choice.parse::<usize>() {
            Ok(int) => {
                if int <= 0 || int > self.options.len() {
                    return None;
                }
                return Some(self.options[int - 1].to_owned());
            }
            Err(_) => {
                for el in self.options {
                    if el == choice {
                        return Some(el.to_owned());
                    }
                }
                return None;
            }
        }
    }

    /// Displays a pretty interface to the options
    pub fn display(&'o self) -> String {
        let mut buffer: Vec<String> = Vec::new();
        for i in 0..self.options.len() {
            buffer.push(format!("{}.) {}", i + 1, self.options[i]));
        }
        buffer.join("\n")
    }
}
