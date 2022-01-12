#[derive(Debug)]
//pub struct StrSplit<'a, 'b> {
pub struct StrSplit<'a, D> { // D is not lifetime, D is Type
    text: Option<&'a str>,
    delimiter: D,
}

//impl<'a, 'b> StrSplit<'a, 'b> {
impl<'a, D> StrSplit<'a, D> {
    pub fn new(text: &'a str,
               //delimiter: &'b str) -> Self {
               delimiter: D) -> Self {
        Self {
            text: Some(text),
            delimiter,
        }
    }
}


pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}


//impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
//impl<'a> Iterator for StrSplit<'a, '_> {
impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        //if let Some(ref mut text) = self.text {
        let local_text = self.text.as_mut()?; // as_mut() is function on Option: Option<T> -> Option<&mut T>
        
        //if let Some(next_delim) = local_text.find(&self.delimiter) {
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(local_text) {
            /*
            let until_delimiter = &local_text[..next_delim];
            *local_text = &local_text[(next_delim + self.delimiter.len())..];
             */
            let until_delimiter = &local_text[..delim_start];
            *local_text = &local_text[delim_end..];
            Some(until_delimiter)
        } else {
            self.text.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self)
            .map(|start| (start, start + self.len()))
    }
}


impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}


/*
pub fn until_char<'s>(s: &'s str,
                  c: char) -> &'s str {
*/
pub fn until_char(s: &str,
                  c: char) -> &'_ str {
    
    //StrSplit::new(s, &format!("{}",c))
    StrSplit::new(s, c)
        .next()
        .expect("should return always something")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("foooking paavel", 'i'), "foook");
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
fn wrong_delimiter() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, "X");
    assert!(letters.eq(vec!["a b c d e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}


fn main() {
    println!("{:#?}",
             //StrSplit::new("foookume is KinG", " ").into_iter().collect::<Vec<_>>(),
             StrSplit::new("foookume is KinG", " ").collect::<Vec<_>>(),
    );
}
