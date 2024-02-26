use serde::Deserialize;
use std::collections::HashMap;
use warp::Filter;
mod self_virtual_dom;
use self_virtual_dom::{update_dom, virtual_dom_to_html, AppResponse, ElementType, VNode};

#[derive(Deserialize)]
struct Input {
    input: String,
}

const HTML_TEMPLATE: &str = include_str!("index.html");

#[tokio::main]
async fn main() {
    let html_route = warp::path::end().map(|| warp::reply::html(HTML_TEMPLATE));

    let run_app_route = warp::path("run_app").map(|| {
        let app_response = run_app("");
        warp::reply::json(&app_response)
    });

    let update_input_route = warp::path("update_input")
        .and(warp::post())
        .and(warp::body::json())
        .map(|input: Input| {
            let app_response = update_input(input.input);
            warp::reply::json(&app_response)
        });
    let routes = warp::any().and(html_route.or(run_app_route).or(update_input_route));

    let addr = ([127, 0, 0, 1], 3030);
    warp::serve(routes).run(addr).await;
}

pub fn run_app(dynamic_input: &str) -> AppResponse {
    let old_dom = VNode {
        element_type: ElementType::Element(
            "div".to_string(),
            HashMap::new(),
            vec![
                ElementType::Text(dynamic_input.to_string()),
                ElementType::Element(
                    "input".to_string(),
                    [("id".to_string(), "myInput".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    vec![],
                ),
            ],
        ),
    };

    let new_dom = VNode {
        element_type: ElementType::Element(
            "div".to_string(),
            HashMap::new(),
            vec![ElementType::Text(dynamic_input.to_string())],
        ),
    };

    // 仮想DOMの更新の差分を取得
    let diff = update_dom(&old_dom, &new_dom);

    diff
}

pub fn update_input(input: String) -> AppResponse {
    let old_dom = VNode {
        element_type: ElementType::Element(
            "div".to_string(),
            HashMap::new(),
            vec![ElementType::Text("".to_string())],
        ),
    };

    let new_dom = VNode {
        element_type: ElementType::Element(
            "div".to_string(),
            HashMap::new(),
            if input.is_empty() {
                vec![]
            } else {
                vec![ElementType::Text(input.clone())]
            },
        ),
    };

    let diff = update_dom(&old_dom, &new_dom);

    let html: String = virtual_dom_to_html(&new_dom.element_type);

    println!("HTML PREVIEW:{:?}", html);

    diff
}
