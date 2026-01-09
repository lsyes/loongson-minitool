use gpui::*;
use gpui_component::{button::*, *};
use std::process::Command;

pub struct MiniTool;

impl Render for MiniTool {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0xffffff))
            .child(
                div()
                    .h_flex()
                    .w_full()
                    .px_4()
                    .py_3()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_lg())
                            .text_color(rgb(0x000000))
                            .font_weight(FontWeight::BOLD)
                            .child("龙芯配置工具集")
                    )
            .p_4()
            .child(
                div()
                    .v_flex()
                    .gap_4()
                    .child(
                        Button::new("remove-app-compat")
                            .label("自动干掉app-compat")
                            .on_click(move |_, _, _| {
                                let command = r#"
                                #!/bin/bash
                                set +e
                                echo "自动化安装并干掉app-compat"

                                echo "正在配置软件源..."

                                wget -qO - https://opensrc.qinyn.eu.org/d/onedrive/loongarch64/mirror/signing-key.gpg.asc \
                                | sudo gpg --dearmor -vo /usr/share/keyrings/elysia-loongarch-archive-keyring.gpg

                                sudo sed -i '$a\deb [signed-by=/usr/share/keyrings/elysia-loongarch-archive-keyring.gpg] \
                                https://opensrc.qinyn.eu.org/d/onedrive/loongarch64/mirror sid main' /etc/apt/sources.list

                                sudo apt update

                                echo "正在更新软件包列表..."
                                sudo apt update
                                echo "正在干掉app-compat......"
                                sudo apt remove -y app-compat-dkms app-compat-libs
                                echo "正在安装liblol"
                                sudo apt install -y liblol
                                echo "Done!"
                                "#;
                                
                                let _ = Command::new("konsole")
                                    .arg("--noclose")
                                    .arg("-e")
                                    .arg("bash") 
                                    .arg("-c") 
                                    .arg(command)
                                    .spawn();
                            })
                    )
                    .child(
                        Button::new("switch-box64")
                            .label("自动切换到box64")
                            .on_click(move |_, _, _| {
                                let command = r#"
                        #!/bin/bash
                        echo "正在切换到 box64..."
                        sudo update-alternatives --set emu-x86_64.conf /usr/lib/binfmt.alternatives/box64.conf 2>/dev/null \
                        && echo "  ✓ x86_64 配置已切换" || echo "  ⚠  x86_64 切换失败"

                        sudo update-alternatives --set emu-i386.conf /usr/lib/binfmt.alternatives/box32.conf 2>/dev/null \
                        && echo "  ✓ i386 配置已切换" || echo "  ⚠  i386 切换失败"

                        sudo systemctl restart systemd-binfmt.service 2>&1 \
                        && echo "  ✓ binfmt 服务已重启" || echo "  ⚠  服务重启失败"
                        echo "    切换完成！"
                        echo ""
                        echo "按 Enter 键关闭终端..."
                        read
                        "#;
                                
                                let _ = Command::new("konsole")
                                    .arg("--noclose")
                                    .arg("-e")
                                    .arg("bash")
                                    .arg("-c")
                                    .arg(command)
                                    .spawn();
                            })
                    )
                    .child(
                        Button::new("author-info")
                            .label("ℹ️关于作者")
                            .on_click(move |_, _, cx| {
                                struct AboutWindow;
                                
                                impl Render for AboutWindow {
                                    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                                        div()
                                            .v_flex()
                                            .gap_4()
                                            .size_full()
                                            .items_center()
                                            .justify_center()
                                            .p_6()
                                            .bg(rgb(0xffffff))
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .text_color(rgb(0x000000))
                                                    .font_weight(FontWeight::BOLD)
                                                    .child("龙芯小白配置工具集")
                                            )
                                            .child(
                                                div()
                                                    .v_flex()
                                                    .gap_1()
                                                    .text_color(rgb(0x000000))
                                                    .child("作者: lsyes")
                                                    .child("版本: v1.0rc1")
                                                    .child("架构: loongarch64")
                                                    .child("@2026")
                                                    .child("框架:gpui,which powerrd by rust")
                                            )
                                    }
                                }
                                
                                let _ = cx.open_window(
                                    WindowOptions {
                                        window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                                            Point::new(px(160.), px(160.)),
                                            size(px(300.), px(200.)),
                                        ))),
                                        titlebar: Some(TitlebarOptions {
                                            title: Some("关于".into()),
                                            ..Default::default()
                                        }),
                                        ..Default::default()
                                    },
                                    |window, cx| {
                                        let view = cx.new(|_| AboutWindow);
                                        cx.new(|cx| Root::new(view, window, cx))
                                    },
                                );
                            })
                    )
            )
    }
}
