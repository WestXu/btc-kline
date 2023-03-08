mod plot;
mod prices;

use wasm_bindgen_futures::spawn_local;

async fn sleep(delay: i32) {
    let mut cb = |resolve: js_sys::Function, _: js_sys::Function| {
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay);
    };

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

fn main() -> std::io::Result<()> {
    spawn_local(async {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let node = document.create_element("div").unwrap();
        body.append_child(node.as_ref()).unwrap();

        let mut prices = prices::Prices::new().await;

        loop {
            let plot = plot::plot(&prices.data).await;
            node.set_inner_html(&plot.to_inline_html(None));

            let script = node
                .get_elements_by_tag_name("script")
                .item(0)
                .unwrap()
                .inner_html();
            js_sys::eval(&script).unwrap();

            sleep(1000).await;
            prices.update().await;
        }
    });

    Ok(())
}
