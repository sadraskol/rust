use std::io;

use crate::custom_markdown::StrWrite;

pub mod java;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SadLang {
    Java,
    Text,
}

type Span<'a> = (usize, usize, &'a str);

pub fn highlight<W: StrWrite>(mut w: W, s: &str, l: SadLang) -> io::Result<()> {
    return match l {
        SadLang::Java => {
            let mut slash: Option<usize> = None;
            let mut size = 0;
            let mut inline_comment = None;
            let mut spans: Vec<Span> = vec![];

            for c in s.chars() {
                if inline_comment.is_some() {
                    println!("some inline comment");
                    if c == '\n' {
                        println!("\\n");
                        spans.push((inline_comment.unwrap(), size, "inline-comment"));
                        inline_comment = None;
                    } else {
                        println!("\\n other");
                        size += 1;
                        continue;
                    }
                }
                if slash.is_some() {
                    println!("some /");
                    if c == '/' {
                        println!("//");
                        inline_comment = Some(slash.unwrap())
                    } else {
                        println!("/ other");
                        slash = None
                    }
                    size += 1;
                    continue;
                }
                if c == '/' {
                    println!("/");
                    slash = Some(size);
                }
                size += 1;
            }

            if inline_comment.is_some() {
                spans.push((inline_comment.unwrap(), size, "inline-comment"));
            }

            let mut mark = 0;
            for span in spans {
                w.write_str(&s[mark..span.0])?;
                w.write_str(format!("<span class=\"h-{}\">", span.2).as_str())?;
                w.write_str(&s[span.0..span.1])?;
                w.write_str("</span>")?;
                mark = span.1;
            }
            w.write_str(&s[mark..])
        }
        SadLang::Text => w.write_str(s)
    }
}

#[cfg(test)]
mod test {
    use crate::highlight::SadLang::{Java, Text};
    use crate::highlight::highlight;

    #[test]
    fn text_provided_as_it_is() {
        let mut s = String::with_capacity(100);
        highlight(&mut s, "some text code\n", Text).unwrap();
        assert_eq!("some text code\n", s.as_str());
    }

    #[test]
    fn malformed_java_is_okay() {
        let mut s = String::with_capacity(100);
        highlight(&mut s, "some java code\n", Java).unwrap();
        assert_eq!("some java code\n", s.as_str());
    }

    #[test]
    fn inline_comment_is_okay() {
        let mut s = String::with_capacity(100);
        highlight(&mut s, "some // java code\n", Java).unwrap();
        assert_eq!("some <span class=\"h-inline-comment\">// java code</span>\n", s.as_str());
    }

    #[test]
    fn inline_comment_works_with_eol_delimiter() {
        let mut s = String::with_capacity(100);
        highlight(&mut s, "some // java code", Java).unwrap();
        assert_eq!("some <span class=\"h-inline-comment\">// java code</span>", s.as_str());
    }
}