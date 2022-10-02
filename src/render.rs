use axum::response::{Html, IntoResponse};
use tracing::error;

macro_rules! render {
    ($template:path) => {{
        use $crate::render::Render;
        Render(move |o| $template(o))
    }};
    ($template:path, $($arg:expr),*) => {{
        use $crate::render::Render;
        Render(move |o| $template(o, $($arg),*))
    }}
}

pub struct Render<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>>(pub T);

impl<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>> IntoResponse for Render<T> {
    fn into_response(self) -> axum::response::Response {
        let mut buf = Vec::new();
        match self.0(&mut buf) {
            Ok(()) => Html(buf).into_response(),
            Err(e) => {
                error!("render failed: {:?}", e);
                "Render failed".into_response()
            }
        }
    }
}
