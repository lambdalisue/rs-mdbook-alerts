use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::PreprocessorContext;
use once_cell::sync::Lazy;
use regex::Regex;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub struct Preprocessor;

impl mdbook::preprocess::Preprocessor for Preprocessor {
    fn name(&self) -> &str {
        "alerts"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut error: Option<Error> = None;
        book.for_each_mut(|item: &mut BookItem| {
            if error.is_some() {
                return;
            }
            if let BookItem::Chapter(ref mut chapter) = *item {
                if let Err(err) = handle_chapter(chapter) {
                    error = Some(err)
                }
            }
        });
        error.map_or(Ok(book), Err)
    }
}

fn handle_chapter(chapter: &mut Chapter) -> Result<(), Error> {
    chapter.content = inject_stylesheet(&chapter.content)?;
    chapter.content = render_alerts(&chapter.content)?;
    Ok(())
}

fn inject_stylesheet(content: &str) -> Result<String, Error> {
    let style = Asset::get("style.css").expect("style.css not found in assets");
    let style = std::str::from_utf8(style.data.as_ref())?;
    Ok(format!("<style>\n{style}\n</style>\n{content}"))
}

fn render_alerts(content: &str) -> Result<String, Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?m)^> \[!(?P<kind>[^\]]+)\]\s*$(?P<body>(?:\n>.*)*)")
            .expect("failed to parse regex")
    });
    let alerts = Asset::get("alerts.tmpl").expect("alerts.tmpl not found in assets");
    let alerts = std::str::from_utf8(alerts.data.as_ref())?;
    let alerts = alerts.replace("\r\n", "\n");
    let newline = find_newline(content);
    let content = content.replace(&newline, "\n");
    let content = content.as_str();
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let kind = caps
            .name("kind")
            .expect("kind not found in regex")
            .as_str()
            .to_lowercase();
        let body = caps
            .name("body")
            .expect("body not found in regex")
            .as_str()
            .replace("\n>\n", "\n\n")
            .replace("\n> ", "\n");
        alerts.replace("{kind}", &kind).replace("{body}", &body)
    });
    Ok(content.replace("\n", newline))
}

fn find_newline(content: &str) -> &'static str {
    let mut cr = 0;
    let mut lf = 0;
    content.chars().for_each(|c| match c {
        '\r' => cr += 1,
        '\n' => lf += 1,
        _ => {}
    });
    return if cr == lf { "\r\n" } else { "\n" };
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use insta::assert_debug_snapshot;
    use insta::assert_snapshot;

    #[test]
    fn test_render_alerts() {
        let content = indoc! {r#"
        This should be a paragraph.

        > This should be a blockquote.
        > This should be in a previous blockquote.

        > [!NOTE]
        > This should be alert.
        > This should be in a previous alert.

        This should be a paragraph.
        "#};
        let result = render_alerts(content).unwrap();
        assert_debug_snapshot!(result);
        assert_snapshot!(result);
    }

    #[test]
    fn test_render_alerts_with_crlf() {
        let content = indoc! {r#"
        This should be a paragraph.

        > This should be a blockquote.
        > This should be in a previous blockquote.

        > [!NOTE]
        > This should be alert.
        > This should be in a previous alert.

        This should be a paragraph.
        "#};
        let content = content.replace("\n", "\r\n");
        let result = render_alerts(content.as_str()).unwrap();
        assert_debug_snapshot!(result);
        assert_snapshot!(result);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_stylesheet_includes_css() {
        let content = "Hello, world!";
        let result = inject_stylesheet(content).unwrap();
        assert!(result.contains("<style>"));
        assert!(result.contains(".mdbook-alerts"));
        assert!(result.contains("</style>"));
        assert!(result.contains("Hello, world!"));
    }

    #[test]
    fn test_render_alerts_basic_alert() {
        let input = "> [!note]\n> This is a note.";
        let output = render_alerts(input).unwrap();
        assert!(output.contains("note"));
        assert!(output.contains("This is a note."));
        assert_eq!(
            output,
            r#"<div class="mdbook-alerts mdbook-alerts-note">
<p class="mdbook-alerts-title">
  <span class="mdbook-alerts-icon"></span>
  note
</p>


This is a note.

</div>
"#
        );
    }

    #[test]
    fn test_render_alerts_multiple_alerts() {
        let input = "> [!warning]\n> Warning 1.\n\n> [!tip]\n> Tip 2.";
        let output = render_alerts(input).unwrap();
        assert!(output.contains("warning"));
        assert!(output.contains("Warning 1."));
        assert!(output.contains("tip"));
        assert!(output.contains("Tip 2."));
        assert_eq!(
            output,
            r#"<div class="mdbook-alerts mdbook-alerts-warning">
<p class="mdbook-alerts-title">
  <span class="mdbook-alerts-icon"></span>
  warning
</p>


Warning 1.

</div>


<div class="mdbook-alerts mdbook-alerts-tip">
<p class="mdbook-alerts-title">
  <span class="mdbook-alerts-icon"></span>
  tip
</p>


Tip 2.

</div>
"#
        );
    }
}
