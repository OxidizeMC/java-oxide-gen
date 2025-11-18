use super::methods::Method;
use crate::{
    config::{ClassConfig, DocConfig},
    parser_util::Id,
};
use cafebabe::descriptors::{FieldDescriptor, FieldType};
use std::fmt::{self, Display, Formatter};

pub struct KnownDocsUrl {
    pub label: String,
    pub url: String,
}

impl Display for KnownDocsUrl {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[{}]({})", &self.label, &self.url)
    }
}

impl KnownDocsUrl {
    pub fn from_class(config: &ClassConfig, java_class: Id) -> Option<KnownDocsUrl> {
        let java_class: &str = java_class.as_str();
        let pattern: &DocConfig = config.doc_pattern?;

        for ch in java_class.chars() {
            match ch {
                'a'..='z' => {}
                'A'..='Z' => {}
                '0'..='9' => {}
                '_' | '$' | '/' => {}
                // Contains invalid charater
                _ch => return None,
            }
        }

        let last_slash: Option<usize> = java_class.rfind('/');
        let no_namespace: &str = if let Some(last_slash) = last_slash {
            &java_class[(last_slash + 1)..]
        } else {
            java_class
        };

        let java_class: String = java_class
            .replace('/', pattern.sep.class_namespace.as_str())
            .replace('$', pattern.sep.class_inner_class.as_str());

        Some(KnownDocsUrl {
            label: no_namespace.to_owned().replace('$', "."),
            url: pattern
                .class_url
                .as_ref()?
                .replace("{CLASS}", java_class.as_str())
                .replace("{CLASS.LOWER}", java_class.to_ascii_lowercase().as_str()),
        })
    }

    pub fn from_method(config: &ClassConfig, method: &Method) -> Option<KnownDocsUrl> {
        let is_constructor: bool = method.java.is_constructor();

        let pattern: &DocConfig = config.doc_pattern?;
        let url_pattern: &String = if is_constructor {
            pattern
                .constructor_url
                .as_ref()
                .or(pattern.method_url.as_ref())?
        } else {
            pattern.method_url.as_ref()?
        };

        for ch in method.class.path().as_str().chars() {
            match ch {
                'a'..='z' => {}
                'A'..='Z' => {}
                '0'..='9' => {}
                '_' | '$' | '/' => {}
                // Contains invalid charater
                _ch => return None,
            }
        }

        let java_class: String = method
            .class
            .path()
            .as_str()
            .replace('/', pattern.sep.class_namespace.as_str())
            .replace('$', pattern.sep.class_inner_class.as_str());

        let java_outer_class: String = method
            .class
            .path()
            .as_str()
            .rsplit('/')
            .next()
            .unwrap()
            .replace('$', pattern.sep.class_inner_class.as_str());

        let java_inner_class: &str = method
            .class
            .path()
            .as_str()
            .rsplit('/')
            .next()
            .unwrap()
            .rsplit('$')
            .next()
            .unwrap();

        let label: &str = if is_constructor {
            java_inner_class
        } else {
            for ch in method.java.name().chars() {
                match ch {
                    'a'..='z' => {}
                    'A'..='Z' => {}
                    '0'..='9' => {}
                    '_' => {}
                    // Contains invalid charater
                    _ch => return None,
                }
            }
            method.java.name()
        };

        let mut java_args: String = String::new();

        let mut prev_was_array: bool = false;
        for arg in method.java.descriptor().parameters.iter() {
            if prev_was_array {
                prev_was_array = false;
                java_args.push_str("[]");
            }

            if !java_args.is_empty() {
                java_args.push_str(&pattern.sep.argument[..]);
            }

            let obj_arg: String;
            java_args.push_str(match arg.field_type {
                FieldType::Boolean => "boolean",
                FieldType::Byte => "byte",
                FieldType::Char => "char",
                FieldType::Short => "short",
                FieldType::Integer => "int",
                FieldType::Long => "long",
                FieldType::Float => "float",
                FieldType::Double => "double",
                FieldType::Object(ref class_name) => {
                    let class: Id<'_> = Id::from(class_name);
                    obj_arg = class
                        .as_str()
                        .replace('/', pattern.sep.argument_namespace.as_str())
                        .replace('$', pattern.sep.argument_inner_class.as_str());
                    obj_arg.as_str()
                }
            });
            if arg.dimensions > 0 {
                for _ in 1..arg.dimensions {
                    java_args.push_str("[]");
                }
                prev_was_array = true; // level 0
            }
        }

        if prev_was_array {
            if method.java.is_varargs() {
                java_args.push_str("...");
            } else {
                java_args.push_str("[]");
            }
        }

        // No {RETURN} support... yet?

        Some(KnownDocsUrl {
            label: label.to_owned(),
            url: url_pattern
                .replace("{CLASS}", java_class.as_str())
                .replace("{CLASS.LOWER}", java_class.to_ascii_lowercase().as_str())
                .replace("{CLASS.OUTER}", java_outer_class.as_str())
                .replace("{CLASS.INNER}", java_inner_class)
                .replace("{METHOD}", label)
                .replace("{ARGUMENTS}", java_args.as_str()),
        })
    }

    pub fn from_field(
        config: &ClassConfig,
        java_class: &str,
        java_field: &str,
        _java_descriptor: FieldDescriptor,
    ) -> Option<KnownDocsUrl> {
        let pattern: &DocConfig = config.doc_pattern?;
        let field_url_pattern: &String = pattern.field_url.as_ref()?;

        for ch in java_class.chars() {
            match ch {
                'a'..='z' => {}
                'A'..='Z' => {}
                '0'..='9' => {}
                '_' | '$' | '/' => {}
                // Contains invalid charater
                _ch => return None,
            }
        }

        for ch in java_field.chars() {
            match ch {
                'a'..='z' => {}
                'A'..='Z' => {}
                '0'..='9' => {}
                '_' => {}
                // Contains invalid charater
                _ch => return None,
            }
        }

        let java_class: String = java_class
            .replace('/', pattern.sep.class_namespace.as_str())
            .replace('$', pattern.sep.class_inner_class.as_str());

        // No {RETURN} support... yet?

        Some(KnownDocsUrl {
            label: java_field.to_owned(),
            url: field_url_pattern
                .replace("{CLASS}", java_class.as_str())
                .replace("{CLASS.LOWER}", java_class.to_ascii_lowercase().as_str())
                .replace("{FIELD}", java_field),
        })
    }
}
