
use regex::{Captures, Regex};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
pub enum XKCDMarkdown {
    Hidden(String),
    Italic(String),
    Bold(String),
    BoldItalic(String),
    Underline(String),
    UnderlineItalic(String),
    UnderlineBold(String),
    UnderlineBoldItalic(String),
    Strikethrough(String),
    Block(String),
    Header(String),
    Code(String),
    Li(u32),
    Br,
    Link{content: String, href: String},
    Plain(String),
    End,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?x)
        (?:\[\[(?P<square>[^\]]*?)\]\])| # Contents in square brackets
        (?:\{\{(?P<curly>[^\}]*?)\}\})| # Contents in curly brackets
        (?:_(?P<under>[^_]*?)_)| # Contents between single underscores
        (?:\[(?P<singlesquare>[^\]]*?)\])| # Contents in single square brackets
        (?:==(?P<equal>[^=]*?)==)| # Contents in equal signs
        (?P<li>\*+)| # Get * tags for li items
        (?:<code>(?P<code>.+)</code>)|
        (?P<br><br>)|
        (?P<plain>[^=\[\]\{\}\*]+) # Contents outside of other groups
    ").unwrap();
}

pub struct MarkdownConverter(pub Vec<XKCDMarkdown>);

impl MarkdownConverter {
    fn xkcd_capture(cap: Captures) -> XKCDMarkdown {
        if let Some(matches) = cap.name("square") {
            return XKCDMarkdown::Bold(matches.as_str().to_string());
        }
        else if let Some(matches) = cap.name("curly") {
            return XKCDMarkdown::Hidden(matches.as_str().to_string());
        }
        else if let Some(matches) = cap.name("under") {
            return XKCDMarkdown::Underline(matches.as_str().to_string());
        }
        else if let Some(matches) = cap.name("plain") {
            return XKCDMarkdown::Plain(matches.as_str().to_string());
        }
        else if let Some(matches) = cap.name("li") {
            // Unable to find single squares outside of Explanation section
            return XKCDMarkdown::Plain(matches.as_str().to_string());
        }
        return XKCDMarkdown::End;

    }
    
    pub fn from_xkcd(s: &str) -> Self {
        let mut result = Vec::new();
        for cap in RE.captures_iter(s) {
            result.push(Self::xkcd_capture(cap));
        }
        MarkdownConverter(result)
    }

    fn explain_capture(cap: Captures) -> XKCDMarkdown {
        if let Some(matches) = cap.name("square") {
            let split = matches.as_str().split("|").collect::<Vec<_>>();
            
            // If len is 1, then it is an internal link where the content is the same as the link suffix
            if split.len() == 1 {
                let href = format!("https://www.explainxkcd.com/wiki/index.php/{}", split[0].replace(" ", "_"));
                let content = split[0].to_string();
                return XKCDMarkdown::Link{content, href,};
            }
            // If len is 2, then it is an internal link where the content is different from the link
            else if split.len() == 2 {
                let href = format!("https://www.explainxkcd.com/wiki/index.php/{}", split[0].replace(" ", "_"));
                let content = split[1].to_string();
                return XKCDMarkdown::Link{content, href,};
            }
            // If len is > 2, it is not expected to be a link, so add as plain and continue
            else {
                return XKCDMarkdown::Plain(matches.as_str().to_string());
            }
        }
        else if let Some(matches) = cap.name("curly") {
            let split = matches.as_str().split("|").collect::<Vec<_>>();
            // If curly brackers have 2 items, this it is most linkly an external link
            if split.len() == 2 {
                // If content before | is w, then it is an external wikipedia link
                if split[0] == "w" {
                    let content = split[1].to_string();
                    let href = format!("https://en.wikipedia.org/wiki/{}", split[1].replace(" ", "_"));
                    return XKCDMarkdown::Link{content, href,};
                }
            }
            // If not len of 2, then it's not an expected link, add as plain
            else {
                return XKCDMarkdown::Hidden(matches.as_str().to_string());
            }
            
        }
        else if let Some(matches) = cap.name("equal") {
            let content = matches.as_str().to_string();
            // Header content is not the explanation, we've gone to far. Return completed Markdown Converter
            if content != "Explanation" {
                return XKCDMarkdown::End;
            }
            return XKCDMarkdown::Header(content);
        }
        else if let Some(matches) = cap.name("singlesquare") {
            // Unable to find single squares outside of Explanation section
            return XKCDMarkdown::Plain(matches.as_str().to_string());
        }
        else if let Some(matches) = cap.name("code") {
            
            // Unable to find single squares outside of Explanation section
            return XKCDMarkdown::Code(matches.as_str().to_string());
        }
        else if let Some(matches) = cap.name("li") {
            // Unable to find single squares outside of Explanation section
            return XKCDMarkdown::Li(matches.as_str().len() as u32);
        }
        else if let Some(_) = cap.name("br") {
            // Unable to find single squares outside of Explanation section
            return XKCDMarkdown::Br;
        }
        else if let Some(matches) = cap.name("plain") {
            return XKCDMarkdown::Plain(matches.as_str().to_string());
        }
        return XKCDMarkdown::End;
    }

    pub fn from_explain(s: &str) -> Self {
        let mut result = Vec::new();
        for cap in RE.captures_iter(s) {
            let md = Self::explain_capture(cap);
            if md == XKCDMarkdown::End {
                return MarkdownConverter(result);
            }
            result.push(md);
        }
        MarkdownConverter(result)
    }
    
    pub fn discord(&self) -> String {
        let mut output = String::new();
        for comp in &self.0 {
            match comp {
                XKCDMarkdown::Hidden(_) => {},
                XKCDMarkdown::Link{content, href} => output += &format!("[{}]({})", content, href),
                XKCDMarkdown::Italic(s) => output += &format!("*{}*", s),
                XKCDMarkdown::Bold(s) => output += &format!("**{}**", s),
                XKCDMarkdown::BoldItalic(s) => output += &format!("***{}***", s),
                XKCDMarkdown::Underline(s) => output += &format!("__{}__", s),
                XKCDMarkdown::UnderlineItalic(s) => output += &format!("__*{}*__", s),
                XKCDMarkdown::UnderlineBold(s) => output += &format!("__**{}**__", s),
                XKCDMarkdown::UnderlineBoldItalic(s) => output += &format!("__***{}***__", s),
                XKCDMarkdown::Strikethrough(s) => output += &format!("~~{}~~", s),
                XKCDMarkdown::Block(s) => output += &format!("`{}`", s),
                XKCDMarkdown::Header(s) => output += &format!("**{}**", s),
                XKCDMarkdown::Plain(s) => output += s,
                XKCDMarkdown::Li(i) => {
                    for _j in 0..*i {
                        output += ">";
                    }
                }
                XKCDMarkdown::Code(s) => output += &format!("`{}`", s),
                XKCDMarkdown::Br => output += "\n",
                XKCDMarkdown::End => {}
                
            }
        }
        output
    }
    
    pub fn html(&self) -> String {
        let mut output = String::new();
        for comp in &self.0 {
            match comp {
                XKCDMarkdown::Hidden(_) => {},
                XKCDMarkdown::Link{content, href} => output += &format!("<a href=\"{}\">{}</a>", href, content),
                XKCDMarkdown::Italic(s) => output += &format!("<i>{}</i>", s),
                XKCDMarkdown::Bold(s) => output += &format!("<b>{}</b>", s),
                XKCDMarkdown::BoldItalic(s) => output += &format!("<b><i>{}</i></b>", s),
                XKCDMarkdown::Underline(s) => output += &format!("<u>{}</u>", s),
                XKCDMarkdown::UnderlineItalic(s) => output += &format!("<u><i>*{}</i></u>", s),
                XKCDMarkdown::UnderlineBold(s) => output += &format!("<u><b>*{}</b></u>", s),
                XKCDMarkdown::UnderlineBoldItalic(s) => output += &format!("<u><b><i>*{}</i></b></u>", s),
                XKCDMarkdown::Strikethrough(s) => output += &format!("<s>{}</s>", s),
                XKCDMarkdown::Block(s) => output += &format!("<blockquote>{}</blockquote>", s),
                XKCDMarkdown::Header(s) => output += &format!("<h1>{}</h1>", s),
                XKCDMarkdown::Plain(s) => output += s,
                XKCDMarkdown::Li(i) => {
                    for _ in 0..*i {
                        output += "<li></li>";
                    }
                }
                XKCDMarkdown::End => {},
                XKCDMarkdown::Code(s) => output += &format!("<code>{}</code>", s),
                XKCDMarkdown::Br => output += "<br>",
            }
        }
        output
    }
}