use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;
use swc_core::common::{
    BytePos, Span,
    comments::{Comment, SingleThreadedComments},
};

static MACRO_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@(?P<namespace>[^:]+):(?P<directive>[^\s\[]+)(?:\s*\[(?P<attrs>[^\]]*)\])?")
        .expect("should construct the regex")
});

static ATTR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?P<key>[^=\s]+)\s*=\s*"(?P<value>[^"]*)"#).expect("should construct the regex")
});

pub struct MacroParser {
    namespace: &'static str,
}

impl MacroParser {
    pub fn new(namespace: &'static str) -> Self {
        MacroParser { namespace }
    }

    pub fn parse(&self, swc_comments: &SingleThreadedComments) -> Vec<MacroNode> {
        let (leading, trailing) = swc_comments.borrow_all();

        let mut macros = Vec::new();
        for (ast_pos, comments) in leading.iter().chain(trailing.iter()) {
            for comment in comments {
                if let Some(macro_node) = self.parse_macro(*ast_pos, comment) {
                    macros.push(macro_node);
                }
            }
        }

        macros.sort_by_key(|m| m.span);
        macros
    }

    fn parse_macro(&self, ast_pos: BytePos, comment: &Comment) -> Option<MacroNode> {
        let caps = MACRO_REGEX.captures_iter(&comment.text).next()?;
        let namespace = caps.name("namespace")?;
        if namespace.as_str() != self.namespace {
            return None;
        }

        let directive = caps.name("directive")?;
        let attrs = caps
            .name("attrs")
            .map(|attrs| {
                let mut attr_map = HashMap::new();
                let caps = ATTR_REGEX.captures_iter(attrs.as_str());
                for cap in caps {
                    let Some(key) = cap.name("key") else {
                        continue;
                    };

                    let Some(value) = cap.name("value") else {
                        continue;
                    };

                    attr_map.insert(key.as_str().to_owned(), value.as_str().to_owned());
                }
                attr_map
            })
            .unwrap_or_default();

        let macro_node = MacroNode {
            span: comment.span,
            ast_pos: ast_pos,
            namespace: namespace.as_str().to_owned(),
            directive: directive.as_str().to_owned(),
            attrs,
        };

        Some(macro_node)
    }
}

#[derive(Debug)]
pub struct MacroNode {
    pub span: Span,
    pub ast_pos: BytePos,
    pub namespace: String,
    pub directive: String,
    pub attrs: HashMap<String, String>,
}
