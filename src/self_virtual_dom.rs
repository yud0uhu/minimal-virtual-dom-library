use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/**
 * 仮想DOMの要素を表す構造体
 */
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Text(String),
    Element(String, HashMap<String, String>, Vec<ElementType>),
}

/**
 * 仮想DOMのノードを表す構造体
 */
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct VirtualNode {
    pub element_type: ElementType,
}

/**
 * 仮想DOMの更新の差分を表す列挙型
 */
#[derive(Debug, Serialize, Deserialize)]
pub enum Diff {
    AddNode(VirtualNode),
    RemoveNode(VirtualNode),
}

impl PartialEq for Diff {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Diff::AddNode(node1), Diff::AddNode(node2)) => node1 == node2,
            (Diff::RemoveNode(node1), Diff::RemoveNode(node2)) => node1 == node2,
            _ => false,
        }
    }
}

/**
 * 仮想DOMの更新の結果を表す構造体
 */
#[derive(Debug, Serialize)]
pub struct AppResponse {
    diff: Vec<Diff>,
    html: String,
}

/**
 * 仮想DOMの更新の差分を取得する関数
 */
pub fn update_dom(old: &VirtualNode, new: &VirtualNode) -> AppResponse {
    let mut diff = Vec::new();

    let removed_nodes = find_removed_nodes(old, new);

    for removed_node in removed_nodes {
        diff.push(Diff::RemoveNode(removed_node.clone()));
    }

    let added_nodes = find_added_nodes(old, new);

    for added_node in added_nodes {
        diff.push(Diff::AddNode(added_node.clone()));
    }

    let html = virtual_dom_to_html(&new.element_type);

    for change in &diff {
        match change {
            Diff::AddNode(node) => println!("Added Node: {:?}", node),
            Diff::RemoveNode(node) => println!("Removed Node: {:?}", node),
        }
    }

    AppResponse { diff, html }
}

/**
* 仮想DOMに追加されたノードを取得する関数
*/
fn find_added_nodes(old: &VirtualNode, new: &VirtualNode) -> Vec<VirtualNode> {
    let mut added_nodes = Vec::new();
    find_added_nodes_recursive(&old.element_type, &new.element_type, &mut added_nodes);
    added_nodes
}

/**
 * 仮想DOMに追加されたノードを再帰的に取得する関数
*/
fn find_added_nodes_recursive(
    old: &ElementType,
    new: &ElementType,
    added_nodes: &mut Vec<VirtualNode>,
) {
    if old != new {
        if !new.is_empty_text_node() {
            added_nodes.push(VirtualNode {
                element_type: new.clone(),
            });
        }
    } else if let ElementType::Element(_, _, old_children) = old {
        if let ElementType::Element(_, _, new_children) = new {
            for (old_child, new_child) in old_children.iter().zip(new_children.iter()) {
                find_added_nodes_recursive(old_child, new_child, added_nodes);
            }
        }
    }
}

/**
 * 仮想DOMの削除されたノードを取得する関数
 */
fn find_removed_nodes(old: &VirtualNode, new: &VirtualNode) -> Vec<VirtualNode> {
    let mut removed_nodes = Vec::new();
    find_removed_nodes_recursive(&old.element_type, &new.element_type, &mut removed_nodes);
    removed_nodes
}

/**
 * 仮想DOMの削除されたノードを再帰的に取得する関数
 */
fn find_removed_nodes_recursive(
    old: &ElementType,
    new: &ElementType,
    removed_nodes: &mut Vec<VirtualNode>,
) {
    if old != new {
        if !old.is_empty_text_node() {
            removed_nodes.push(VirtualNode {
                element_type: old.clone(),
            });
        }
    } else if let ElementType::Element(_, _, old_children) = old {
        if let ElementType::Element(_, _, new_children) = new {
            for (old_child, new_child) in old_children.iter().zip(new_children.iter()) {
                find_removed_nodes_recursive(old_child, new_child, removed_nodes);
            }
        }
    }
}

/**
* 仮想DOMの要素が空のテキストノードかどうかを判定する関数
*/
impl ElementType {
    fn is_empty_text_node(&self) -> bool {
        if let ElementType::Text(text) = self {
            text.is_empty()
        } else {
            false
        }
    }
}

/**
 * 仮想DOMの要素をHTMLに変換する関数
 */
pub fn virtual_dom_to_html(node: &ElementType) -> String {
    match node {
        ElementType::Text(text) => text.clone(),
        ElementType::Element(tag, attrs, children) => {
            let attrs_str = attrs
                .iter()
                .map(|(key, value)| format!("{}=\"{}\"", key, value))
                .collect::<Vec<_>>()
                .join(" ");
            let children_str = children
                .iter()
                .map(|child| virtual_dom_to_html(child))
                .collect::<Vec<_>>()
                .join("");
            format!("<{} {}>{}</{}>", tag, attrs_str, children_str, tag)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_dom() {
        let old_dom = VirtualNode {
            element_type: ElementType::Element(
                "div".to_string(),
                HashMap::new(),
                vec![ElementType::Text("Hello".to_string())],
            ),
        };

        let new_dom = VirtualNode {
            element_type: ElementType::Element(
                "div".to_string(),
                HashMap::new(),
                vec![
                    ElementType::Text("World".to_string()),
                    ElementType::Element(
                        "span".to_string(),
                        HashMap::new(),
                        vec![ElementType::Text("!".to_string())],
                    ),
                ],
            ),
        };

        let expected_diff = vec![
            Diff::RemoveNode(VirtualNode {
                element_type: ElementType::Element(
                    "div".to_string(),
                    HashMap::new(),
                    vec![ElementType::Text("Hello".to_string())],
                ),
            }),
            Diff::AddNode(VirtualNode {
                element_type: ElementType::Element(
                    "div".to_string(),
                    HashMap::new(),
                    vec![
                        ElementType::Text("World".to_string()),
                        ElementType::Element(
                            "span".to_string(),
                            HashMap::new(),
                            vec![ElementType::Text("!".to_string())],
                        ),
                    ],
                ),
            }),
        ];
        let app_response = update_dom(&old_dom, &new_dom);

        assert!(app_response.diff == expected_diff);
    }

    #[test]
    fn test_virtual_dom_to_html() {
        let element = ElementType::Element(
            "div".to_string(),
            HashMap::new(),
            vec![
                ElementType::Text("Hello".to_string()),
                ElementType::Element(
                    "span".to_string(),
                    HashMap::new(),
                    vec![ElementType::Text("World".to_string())],
                ),
            ],
        );

        let expected_html = r#"<div >Hello<span >World</span></div>"#;

        let generated_html = virtual_dom_to_html(&element);
        assert_eq!(generated_html, expected_html);
    }
}
