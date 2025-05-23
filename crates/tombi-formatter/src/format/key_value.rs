use std::fmt::Write;

use tombi_ast::AstNode;

use crate::Format;

impl Format for tombi_ast::KeyValue {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        self.leading_comments().collect::<Vec<_>>().format(f)?;

        f.write_indent()?;
        self.keys().unwrap().format(f)?;

        write!(f, " = ")?;

        f.skip_indent();
        self.value().unwrap().format(f)?;

        // NOTE: tailing comment is output by `value.fmt(f)`.

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_format;

    test_format! {
        #[test]
        fn bare_key_value1(r#"key = "value""#) -> Ok("key = \"value\"");
    }
    test_format! {
        #[test]
        fn bare_key_value2(r#"key    = "value""#) -> Ok("key = \"value\"");
    }
    test_format! {
        #[test]
        fn dotted_keys_value1(r#"key1.key2.key3 = "value""#) -> Ok(source);
    }
    test_format! {
        #[test]
        fn dotted_keys_value2(r#"site."google.com" = true"#) -> Ok(source);
    }
    test_format! {
        #[test]
        fn key_value_with_comment(
            r#"
            # leading comment1
            # leading comment2
            key = "value"  # tailing comment
            "#
        ) -> Ok(source);
    }
}
