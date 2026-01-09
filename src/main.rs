use crate::ui::MiniTool;
use gpui::*;
use gpui_component::*;

mod ui;

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Point::new(px(100.), px(100.)),
                    size(px(450.), px(300.)),
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("龙芯配置工具".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| MiniTool);
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
