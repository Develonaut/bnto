// Tree-walking writer — recursively walks roxmltree nodes and writes
// optimized output to xmlwriter, applying passes from OptimizeConfig.

use xmlwriter::XmlWriter;

use super::OptimizeConfig;
use super::xml_passes;

/// Recursively write a node and its children, applying optimization passes.
pub fn write_node(
    node: roxmltree::Node,
    writer: &mut XmlWriter,
    config: &OptimizeConfig,
    editor_prefixes: &[String],
) {
    match node.node_type() {
        roxmltree::NodeType::Root => {
            for child in node.children() {
                write_node(child, writer, config, editor_prefixes);
            }
        }
        roxmltree::NodeType::Element => {
            write_element(node, writer, config, editor_prefixes);
        }
        roxmltree::NodeType::Text => {
            write_text(node, writer, config);
        }
        roxmltree::NodeType::Comment => {
            if !config.remove_comments {
                if let Some(text) = node.text() {
                    writer.write_comment(text);
                }
            }
        }
        roxmltree::NodeType::PI => {
            // Processing instructions (<?xml ...?>) — skip in optimized output.
        }
    }
}

/// Write a text node, minifying whitespace if configured.
fn write_text(node: roxmltree::Node, writer: &mut XmlWriter, config: &OptimizeConfig) {
    let text = node.text().unwrap_or("");
    if config.minify {
        let trimmed = xml_passes::minify_text(text);
        if !trimmed.is_empty() {
            writer.write_text(&trimmed);
        }
    } else {
        writer.write_text(text);
    }
}

/// Write an element node, applying attribute and structural passes.
fn write_element(
    node: roxmltree::Node,
    writer: &mut XmlWriter,
    config: &OptimizeConfig,
    editor_prefixes: &[String],
) {
    let tag = node.tag_name();

    // Pass: remove editor namespace elements (sodipodi:namedview, etc.)
    if xml_passes::is_editor_element(tag, editor_prefixes) {
        return;
    }

    // Pass: remove <metadata> elements
    if config.remove_metadata && tag.name() == "metadata" {
        return;
    }

    // Collect surviving children (to check for group collapsing)
    let children: Vec<roxmltree::Node> = node
        .children()
        .filter(|c| !should_skip_child(c, config, editor_prefixes))
        .collect();

    // Pass: remove empty containers (<g>, <defs> with no surviving children)
    if config.collapse_groups
        && xml_passes::is_collapsible_container(tag.name())
        && children.is_empty()
    {
        return;
    }

    // Pass: collapse single-child <g> with no meaningful attributes
    if config.collapse_groups
        && tag.name() == "g"
        && children.len() == 1
        && xml_passes::has_no_meaningful_attributes(&node, editor_prefixes)
    {
        write_node(children[0], writer, config, editor_prefixes);
        return;
    }

    write_element_tag(node, writer, config, editor_prefixes);
}

/// Write the element's opening tag, attributes, children, and closing tag.
fn write_element_tag(
    node: roxmltree::Node,
    writer: &mut XmlWriter,
    config: &OptimizeConfig,
    editor_prefixes: &[String],
) {
    let elem_name = node.tag_name().name().to_string();
    writer.start_element(&elem_name);

    write_attributes(&node, writer, editor_prefixes);
    write_namespace_declarations(&node, writer, editor_prefixes);

    for child in node.children() {
        write_node(child, writer, config, editor_prefixes);
    }
    writer.end_element();
}

/// Write non-editor, non-empty attributes for an element.
fn write_attributes(node: &roxmltree::Node, writer: &mut XmlWriter, editor_prefixes: &[String]) {
    for attr in node.attributes() {
        if xml_passes::is_editor_attribute(&attr, editor_prefixes) {
            continue;
        }
        if attr.value().is_empty() {
            continue;
        }
        let attr_name = format_attr_name(&attr);
        // Skip editor xmlns declarations
        if attr_name.starts_with("xmlns:") {
            let prefix = &attr_name[6..];
            if editor_prefixes.contains(&prefix.to_string()) {
                continue;
            }
        }
        writer.write_attribute(&attr_name, attr.value());
    }
}

/// Check if a child node would be skipped by passes (for empty container detection).
fn should_skip_child(
    node: &roxmltree::Node,
    config: &OptimizeConfig,
    editor_prefixes: &[String],
) -> bool {
    match node.node_type() {
        roxmltree::NodeType::Element => {
            let tag = node.tag_name();
            if xml_passes::is_editor_element(tag, editor_prefixes) {
                return true;
            }
            if config.remove_metadata && tag.name() == "metadata" {
                return true;
            }
            false
        }
        roxmltree::NodeType::Comment => config.remove_comments,
        roxmltree::NodeType::Text => {
            if config.minify {
                let text = node.text().unwrap_or("");
                xml_passes::minify_text(text).is_empty()
            } else {
                false
            }
        }
        roxmltree::NodeType::PI => true,
        _ => false,
    }
}

/// Format an attribute name, preserving namespace prefixes.
fn format_attr_name(attr: &roxmltree::Attribute) -> String {
    if let Some(ns) = attr.namespace() {
        if let Some(prefix) = namespace_uri_to_prefix(ns) {
            return format!("{prefix}:{}", attr.name());
        }
    }
    attr.name().to_string()
}

/// Write xmlns declarations for namespaces that aren't editor cruft.
fn write_namespace_declarations(
    node: &roxmltree::Node,
    writer: &mut XmlWriter,
    editor_prefixes: &[String],
) {
    for ns in node.namespaces() {
        let uri = ns.uri();
        if let Some(prefix) = ns.name() {
            if editor_prefixes.contains(&prefix.to_string()) {
                continue;
            }
            writer.write_attribute(&format!("xmlns:{prefix}"), uri);
        } else {
            writer.write_attribute("xmlns", uri);
        }
    }
}

/// Map a namespace URI back to its conventional prefix.
fn namespace_uri_to_prefix(uri: &str) -> Option<&'static str> {
    match uri {
        "http://www.w3.org/1999/xlink" => Some("xlink"),
        "http://www.w3.org/XML/1998/namespace" => Some("xml"),
        _ => None,
    }
}
