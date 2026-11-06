//! Supplies a module to the example app's own Engine. The component draws
//! nothing; it only demonstrates the desktop chunk-loading bridge.

use std::rc::Rc;

use nsh_core::JsonValue;
use serde_json::json;
#[cfg(target_os = "linux")]
use nsh_platform_linux as platform;
#[cfg(target_os = "macos")]
use nsh_platform_macos as platform;
#[cfg(windows)]
use nsh_platform_windows as platform;
use platform::{Chunk, ComponentContext, Components, NativeComponent};

const NAME: &str = "rust-exports.chunk-probe";
const CHUNK_ID: &str = "rust-exports.chunk-probe.js";
const SOURCE: &str = r#"export function message() {
  return "Chunk loaded in the host Engine";
}"#;

struct ChunkProbe {
    context: ComponentContext,
    #[cfg(target_os = "linux")]
    widget: platform::gtk4::Widget,
    #[cfg(target_os = "macos")]
    view: platform::objc2::rc::Retained<platform::objc2_app_kit::NSView>,
}

impl ChunkProbe {
    fn new(context: ComponentContext) -> Self {
        #[cfg(target_os = "linux")]
        let widget = platform::gtk4::DrawingArea::new().into();
        #[cfg(target_os = "macos")]
        let view = {
            let mtm = platform::objc2::MainThreadMarker::new()
                .expect("native components are created on the main thread");
            platform::objc2_app_kit::NSView::new(mtm)
        };
        Self {
            context,
            #[cfg(target_os = "linux")]
            widget,
            #[cfg(target_os = "macos")]
            view,
        }
    }

    fn load(&self) {
        self.context.chunk_loaded(Chunk::source(CHUNK_ID, SOURCE.to_owned()));
        self.context.emit("ready", json!({ "chunkId": CHUNK_ID }));
    }
}

impl NativeComponent for ChunkProbe {
    #[cfg(target_os = "linux")]
    fn widget(&self) -> platform::gtk4::Widget {
        self.widget.clone()
    }

    #[cfg(target_os = "macos")]
    fn view(&self) -> platform::objc2::rc::Retained<platform::objc2_app_kit::NSView> {
        self.view.clone()
    }

    fn set_prop(&mut self, _name: &str, _value: Option<&JsonValue>) {}

    fn command(&mut self, name: &str, _args: &JsonValue) {
        if name == "load" {
            self.load();
        }
    }
}

pub(crate) fn register(components: &Components) {
    components.register(NAME, Rc::new(|context| Box::new(ChunkProbe::new(context))));
}
