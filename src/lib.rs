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
    Ok(content.into())
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
}
