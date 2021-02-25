#[cfg(test)]
mod tests {
    use xkcd_utils::xkcd_regex::MarkdownConverter;
    #[test]
    fn regex_test_curly_xkcd() {
        let input = "{{hello}}";
        let markdown = MarkdownConverter::from_xkcd(input);
        assert_eq!(markdown.discord(), "");
        assert_eq!(markdown.html(), "");
    }
    #[test]
    fn regex_test_square_xkcd() {
        let input = "[[Hello World]]";
        // Should become "Hello World" in bold
        let markdown = MarkdownConverter::from_xkcd(input);
        assert_eq!(markdown.discord(), "**Hello World**");
        assert_eq!(markdown.html(), "<b>Hello World</b>");
    }

    #[test]
    fn regex_test_multiple_xkcd() {
        let input: &str = "{{well}} hello [[there]]";
        // "{{well}}"" should become hidden, " hello " should be plain text, and "[[there]]" shoudl become bold
        let markdown = MarkdownConverter::from_xkcd(input);
        assert_eq!(markdown.discord(), " hello **there**");
        assert_eq!(markdown.html(), " hello <b>there</b>");
    }
    #[test]
    fn test_link_explain() {
        let input: &str = "[[here]] {{w|Super Nintendo Entertainment System}}";
        // "{{well}}"" should become hidden, " hello " should be plain text, and "[[there]]" shoudl become bold
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), "[here](https://www.explainxkcd.com/wiki/index.php/here) [Super Nintendo Entertainment System](https://en.wikipedia.org/wiki/Super_Nintendo_Entertainment_System)");
        assert_eq!(markdown.html(), "<a href=\"https://www.explainxkcd.com/wiki/index.php/here\">here</a> <a href=\"https://en.wikipedia.org/wiki/Super_Nintendo_Entertainment_System\">Super Nintendo Entertainment System</a>");
    }
    #[test]
    fn test_links_explain() {
        let input: &str = "[[here]] {{w|Super Nintendo Entertainment System}}";
        // "{{well}}"" should become hidden, " hello " should be plain text, and "[[there]]" shoudl become bold
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), "[here](https://www.explainxkcd.com/wiki/index.php/here) [Super Nintendo Entertainment System](https://en.wikipedia.org/wiki/Super_Nintendo_Entertainment_System)");
        assert_eq!(markdown.html(), "<a href=\"https://www.explainxkcd.com/wiki/index.php/here\">here</a> <a href=\"https://en.wikipedia.org/wiki/Super_Nintendo_Entertainment_System\">Super Nintendo Entertainment System</a>");
    }

    #[test]
    fn hide_header_explain() {
        let input: &str = "{{comic\n| number    = 50\n| date      = January 17, 2006\n| title     = Penny Arcade\n| image     = penny_arcade.jpg\n| titletext = Of course, Penny Arcade has already mocked themselves for this. They don't care.\n}}\n\n==Explanation==";
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), "\n\n**Explanation**");
        assert_eq!(markdown.html(), "\n\n<h1>Explanation</h1>");
    }

    #[test]
    fn li_items() {
        let input = "*\n**";
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), ">\n>>");
    }
    #[test]
    fn li_mixed() {
        let input = "* [[here]]\n* [[there]]";
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), "> [here](https://www.explainxkcd.com/wiki/index.php/here)\n> [there](https://www.explainxkcd.com/wiki/index.php/there)");
    }
    
    #[test]
    fn br() {
        let input = "<br>";
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), "\n");
        assert_eq!(markdown.html(), "<br>");
    }

    #[test]
    fn code_blocks() {
        let input = "<code>print(\"Hello World\")</code>";
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), "`print(\"Hello World\")`");
        assert_eq!(markdown.html(), "<code>print(\"Hello World\")</code>");
    }
    #[test]
    fn code_br_mix() {
        let input = "<br><code>INSERT INTO Students (firstname) VALUES ('Robert');DROP TABLE Students;--&nbsp;');</code>";
        let markdown = MarkdownConverter::from_explain(input);
        assert_eq!(markdown.discord(), "\n`INSERT INTO Students (firstname) VALUES ('Robert');DROP TABLE Students;--&nbsp;');`");
        assert_eq!(markdown.html(), "<br><code>INSERT INTO Students (firstname) VALUES ('Robert');DROP TABLE Students;--&nbsp;');</code>");
    }
    #[test]
    fn never_fail_temp() {
        let input = r#"{{comic
            | number    = 327
            | date      = October 10, 2007
            | title     = Exploits of a Mom
            | image     = exploits_of_a_mom.png
            | titletext = Her daughter is named Help I'm trapped in a driver's license factory.
            }}
            
            ==Explanation==
            [[Mrs. Roberts]] receives a call from her [[Little Bobby Tables|son]]'s school. The caller, likely one of the school's administrators, asks if she really named her son <code>Robert'); DROP TABLE Students;--</code>, a rather unusual name. Perhaps surprisingly, Mrs. Roberts responds in the affirmative, claiming that she uses the nickname "Little Bobby Tables." As the full name is read into the school's system's databases without {{w|Data sanitization#SQL injection|data sanitization}}, it causes the "Students" table in the database to be dropped, meaning it gets deleted.
            
            The title of this comic is a pun. ''Exploit'' can mean an accomplishment or heroic deed, but in computer science, the term refers to a program or technique that takes advantage of a vulnerability in other software. In fact, one could say that her exploit is to exploit an exploit (her achievement is to make use of a vulnerability). The title can also refer to her choice of name for her son, which is rather extraordinary.
            
            In {{w|SQL}}, a database programming language, commands are separated by semicolons <code>;</code>, and strings of text are often delimited using single quotes <code>'</code>. Parts of commands may also be enclosed in parentheses <code>(</code> and <code>)</code>.  Data entries are stored as "rows" within named "tables" of similar items (e.g., <code>Students</code>). The command to delete an entire table (and thus every row of data in that table) is <code>DROP TABLE</code>, as in <code>DROP TABLE Students;</code>.
            
            The exploited vulnerability here is that the single quote in the name input was not correctly "escaped" by the software. That is, if a student's name did indeed contain a quote mark, it should have been read as one of the characters making up the text string and not as the marker to close the string, which it erroneously was. Lack of careful parsing is a common SQL vulnerability; this type of exploit is referred to as {{w|SQL injection}}. Mrs. Roberts thus reminds the school to make sure that they have added data filtering code to prevent code injection exploits in the future.
            
            For example, to add information about Elaine to a data table called 'Students', the SQL query could be:
            <br><code>INSERT INTO Students (firstname) VALUES ('Elaine');</code>
            
            However, using the odd name <code>Robert');DROP TABLE Students;--&nbsp;</code> where we used "Elaine" above, the SQL query becomes:
            <br><code>INSERT INTO Students (firstname) VALUES ('Robert');DROP TABLE Students;--&nbsp;');</code>
            
            
            By insertion of the two semi-colons in the odd name, this is now three well-formed SQL commands:
            <br><code>
            INSERT INTO Students (firstname) VALUES ('Robert');
            
            DROP TABLE Students;
            
            --&nbsp;');
            </code>
            
            The first line is valid SQL code that will legitimately insert data about a student named Robert.
            
            The second line is valid injected SQL code that will delete the whole Student data table from the database.
            
            The third line is a valid code comment (<code>--&nbsp;</code> denotes a comment), which will will cause the rest of the line to be ignored by the SQL server.
            
            For this to work, it helps to know the structure of the database. But it's quite a good guess that a school's student management database might have a table named <code>Students</code>.
            
            Of course, in real life, most exploits of this kind would be performed not by engineering a person's name such that it would eventually be entered into a school database query, but rather by accessing some kind of input system (such as a website's login screen or search interface) and guessing various combinations by trial and error until something works, perhaps by first trying to inject the <code>SHOW TABLES;</code> command to see how the database is structured.
            
            To correctly and harmlessly include the odd name in the Students table in the school database the correct SQL is:
            <br><code>INSERT INTO Students (firstname) VALUES ('Robert<nowiki>''</nowiki>);DROP TABLE Students;--&nbsp;');</code>
            
            Note that the single quote after Robert is now sanitized by doubling it, which changes it from malicious code to harmless data, and the full first 'name' of the student <code>Robert';DROP TABLE Students;--</code> is now stored correctly.
            
            It should be noted that while data sanitization can mitigate the risks of SQL injection, the proper prevention technique is to use {{w|Prepared statement}}s.
            
            Noting the difference between the "actual" name using the word TABLE and the child's nickname being Bobby Tables, one could argue that there's an implied reference to one of the most argued topics of database naming conventions - should table names be singular or plural.
            
            The title text references that Mrs. Roberts' daughter is named "Help I'm trapped in a driver's license factory". This is a play on how if someone is stuck and forced to work in a manufacturing factory/plant, then they will write on the product "Help I am trapped in a ____ factory" in order to tell people on the outside. Having this name would cause any police officer who pulls her over to show some concern. And getting the license in the first place would likely be difficult. The idea of inserting a help message like this was already used in [[10: Pi Equals]].
            <!-- Help! I'm being held prisoner in a Wiki markup code editing facility! -->"#;
        let markdown = MarkdownConverter::from_explain(input);
        println!("{:#?}", markdown.0);
        // println!("Discord Markdown:{}", markdown.discord());
    }
}
