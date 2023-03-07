mod plot;
mod prices;

use wasm_bindgen_futures::spawn_local;

fn main() -> std::io::Result<()> {
    spawn_local(async {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let node = document.create_element("div").unwrap();

        let plot = plot::plot().await;

        node.set_inner_html(&plot.to_inline_html(None));
        body.append_child(node.as_ref()).unwrap();
        let script = node
            .get_elements_by_tag_name("script")
            .item(0)
            .unwrap()
            .inner_html();
        js_sys::eval(&script).unwrap();
    });

    Ok(())
}
