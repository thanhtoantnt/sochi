//! Module containing utility functions handling string

/// Trait containing utility functions for the `String` data structure.
pub trait StringExt<'a> {
    /// Check if a string contains the new line character.
    fn has_new_line(&self) -> bool;

    /// Indent all lines of a string.
    fn indent(&self, indent: usize) -> Self;

    /// Indent all tail lines of a string (exclude the first line).
    fn indent_tail_lines(&self, indent: usize) -> Self;

    /// Indent and add prefix to all tail lines of a string (exclude the first
    /// line).
    fn indent_and_prefix_tail_lines(&self, indent: usize, prefix: &str)
        -> Self;

    /// Process all lines of a string.
    ///
    /// The second parameter `f` of this function is a function which takes the
    /// line number (indexed from 0) and the content and produce a new line.
    fn process_lines(&self, f: fn(usize, String) -> String) -> Self;

    /// Add a prefix to a string if it is not empty.
    fn add_prefix_if_not_empty(self, prefix: &str) -> Self;

    /// Add prefix and suffix to a string.
    fn add_prefix_and_suffix(self, prefix: &str, suffix: &str) -> Self;

    /// Check if a string is enclosed by brackets
    fn is_bracket_enclosed(&self) -> bool;

    /// Wrap long lines to fit a max line width.
    fn wrap_long_lines(&self, column: usize) -> Self;

    /// Wrap and indent long lines to fit a max line width.
    fn wrap_and_indent_long_lines(
        &self,
        max_width: usize,
        indent: usize,
        skip_indent_first_line: bool,
    ) -> Self;
}

/// Implement trait `StringExt` for `String`.
impl<'a> StringExt<'a> for String {
    /// Check if a string contains the new line character.
    fn has_new_line(&self) -> bool {
        self.contains('\n')
    }

    fn indent(&self, indent: usize) -> Self {
        self.lines()
            .map(|line| {
                if line.is_empty() {
                    line.to_string()
                } else {
                    format!("{}{}", " ".repeat(indent), line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn indent_tail_lines(&self, indent: usize) -> Self {
        self.lines()
            .enumerate()
            .map(|(idx, line)| {
                if line.is_empty() || idx == 0 {
                    line.to_string()
                } else {
                    format!("{}{}", " ".repeat(indent), line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn indent_and_prefix_tail_lines(
        &self,
        indent: usize,
        prefix: &str,
    ) -> Self {
        self.lines()
            .enumerate()
            .map(|(idx, line)| {
                if line.is_empty() || idx == 0 {
                    line.to_string()
                } else if indent > 0 {
                    format!("{}{}{}", prefix, " ".repeat(indent), line)
                } else {
                    format!("{}{}", prefix, line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn process_lines(&self, f: fn(usize, String) -> String) -> Self {
        self.lines()
            .enumerate()
            .map(|(idx, line)| f(idx, line.to_string()))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn add_prefix_if_not_empty(self, prefix: &str) -> Self {
        if self.is_empty() {
            self
        } else {
            prefix.to_owned() + &self
        }
    }

    fn add_prefix_and_suffix(self, prefix: &str, suffix: &str) -> Self {
        format!("{prefix}{self}{suffix}")
    }

    fn is_bracket_enclosed(&self) -> bool {
        let tmp = self.trim();
        (tmp.starts_with('[') && tmp.ends_with(']'))
            || (tmp.starts_with('(') && tmp.ends_with(')'))
            || (tmp.starts_with('{') && tmp.ends_with('}'))
    }

    fn wrap_long_lines(&self, max_width: usize) -> Self {
        let wrap_options = textwrap::Options::new(max_width)
            .word_separator(textwrap::WordSeparator::AsciiSpace)
            .word_splitter(textwrap::WordSplitter::NoHyphenation);
        textwrap::wrap(self, &wrap_options).join("\n")
    }

    fn wrap_and_indent_long_lines(
        &self,
        max_width: usize,
        indent: usize,
        skip_indent_first_line: bool,
    ) -> Self {
        let wrap_options = textwrap::Options::new(max_width)
            .word_separator(textwrap::WordSeparator::AsciiSpace)
            .word_splitter(textwrap::WordSplitter::NoHyphenation);
        let res = textwrap::wrap(self, &wrap_options);
        res.into_iter()
            .enumerate()
            .map(|(idx, line)| {
                if line.is_empty() || (skip_indent_first_line && idx == 0) {
                    line.to_string()
                } else {
                    format!("{}{}", " ".repeat(indent), line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
