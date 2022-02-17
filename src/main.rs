#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate passwords;
use nwg::NativeUi;
use passwords::PasswordGenerator;

#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    pass_area: nwg::TextInput,
    copy_button: nwg::Button,
    new_button: nwg::Button,
    about_button: nwg::Button,
}

impl BasicApp {
    fn generator_password() -> String {
        let pg = PasswordGenerator {
            length: 20,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        };
        return pg.generate_one().unwrap();
    }

    fn clipboard_text(&self) {
        nwg::Clipboard::set_data_text(&self.window, &self.pass_area.text());
        let text = nwg::Clipboard::data_text(&self.window);
        assert!(text.is_some());
        assert!(text.unwrap() == self.pass_area.text());
        nwg::modal_info_message(
            &self.window,
            "Copied to clipboard",
            &format!("Copied to clipboard {}", self.pass_area.text()),
        );
    }

    fn about_text(&self) {
        nwg::modal_info_message(&self.window, "About", "© Victor Vinogradov 2022 \nvict.programmer@gmail.com \nhttps://github.com/vict-programmer");
    }

    fn update_text(&self) {
        nwg::TextInput::set_text(&self.pass_area, &BasicApp::generator_password());
    }

    fn close_app(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use super::*;
    use native_windows_gui as nwg;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((400, 260))
                .position((300, 300))
                .title("🔐 Simple Password Generator")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .size((380, 30))
                .position((10, 10))
                .text(&BasicApp::generator_password())
                .readonly(true)
                .parent(&data.window)
                .focus(true)
                .build(&mut data.pass_area)?;

            nwg::Button::builder()
                .size((380, 50))
                .position((10, 50))
                .text("Copy password")
                .parent(&data.window)
                .build(&mut data.copy_button)?;

            nwg::Button::builder()
                .size((380, 50))
                .position((10, 110))
                .text("Generate new password")
                .parent(&data.window)
                .build(&mut data.new_button)?;

            nwg::Button::builder()
                .size((380, 50))
                .position((10, 170))
                .text("About")
                .parent(&data.window)
                .build(&mut data.about_button)?;

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if handle == ui.copy_button {
                                BasicApp::clipboard_text(&ui);
                            } else if handle == ui.new_button {
                                BasicApp::update_text(&ui);
                            } else if handle == ui.about_button {
                                BasicApp::about_text(&ui);
                            }
                        }
                        E::OnWindowClose => {
                            if handle == ui.window {
                                BasicApp::close_app(&ui);
                            }
                        }
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

            Ok(ui)
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
