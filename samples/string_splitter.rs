#[derive(Debug)]
pub struct StrSplit<'a> {
    //text: Option<&'a str>,
    text: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(text: &'a str,
               delimiter: &'a str) -> Self {
        Self {
            //Some<text>,
            text,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.text.find(self.delimiter) {
            let until_delimiter = &self.text[..next_delim];
            self.text = &self.text[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.text.is_empty() {
            None
        } else {
            let rest = self.text;
            self.text = "";
            Some(rest)
        }
    }
}


#[test]
fn it_works() {
    let haystack = "a b c d e";
    /*
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    */
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn will_fail() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, "X");
    assert!(letters.eq(vec!["a b c d e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}


fn main() {
    println!("{:#?}",
             //StrSplit::new("foookume is KinG", " ").into_iter().collect::<Vec<_>>(),
             StrSplit::new("foookume is KinG", " ").collect::<Vec<_>>(),
    );
}
