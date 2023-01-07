const ARRAY_SELECTOR: &'static str = "[]";

type Result<T> = std::result::Result<T, &'static str>;

// Could be replaced by JsonPath. But keeping it simple for now
#[derive(Debug)]
pub struct FieldPath<'a> {
    // Path is a chain of selectors
    pub selectors: Vec<Selector<'a>>,
    pub path_str: &'a str,
}

impl<'a> FieldPath<'a> {
    pub fn parse(path: &'a str) -> Result<Self> {
        let mut buff = path.split('.').filter(|p| !p.is_empty()).into_iter();

        let mut selectors = Vec::new();

        while let Some(token) = buff.next() {
            if ARRAY_SELECTOR == token {
                if let Some(next_token) = buff.next() {
                    match next_token.parse::<u32>() {
                        Ok(index) => selectors.push(Selector::IntoArray(index as usize)),
                        Err(_) => return Err("[] should follow by integer"),
                    }
                } else {
                    return Err("path ending with []");
                }
            } else {
                selectors.push(Selector::Field(token))
            }
        }

        Ok(Self {
            selectors,
            path_str: path,
        })
    }
}

#[derive(Debug)]
pub enum Selector<'a> {
    Field(&'a str),   // .name
    IntoArray(usize), // .[].1
}
