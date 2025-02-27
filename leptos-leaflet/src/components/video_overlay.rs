use leptos::logging::log;
use leptos::prelude::*;

use crate::core::IntoThreadSafeJsValue;

use super::{Bounds, LeafletMapContext};

/// A video overlay component.
#[component(transparent)]
pub fn VideoOverlay(
    #[prop(into)] url: String,
    #[prop(into)] bounds: Bounds,
    #[prop(into, optional)] opacity: Option<Signal<f64>>,
    #[prop(into, optional)] alt: Option<Signal<String>>,
    #[prop(into, optional)] interactive: Option<Signal<bool>>,
    #[prop(into, optional)] cross_origin: Option<Signal<String>>,
    #[prop(into, optional)] cross_origin_toggle: Option<Signal<bool>>,
    #[prop(into, optional)] error_overlay_url: Option<Signal<String>>,
    #[prop(into, optional)] z_index: Option<Signal<f64>>,
    #[prop(into, optional)] class_name: Option<Signal<String>>,
    #[prop(into, optional)] bubbling_mouse_events: Option<Signal<bool>>,
    #[prop(into, optional)] pane: Option<Signal<String>>,
    #[prop(into, optional)] attribution: Option<Signal<String>>,
    #[prop(into, optional)] autoplay: Option<Signal<bool>>,
    #[prop(into, optional)] looped: Option<Signal<bool>>,
    #[prop(into, optional)] keep_aspect_ratio: Option<Signal<bool>>,
    #[prop(into, optional)] muted: Option<Signal<bool>>,
    #[prop(into, optional)] plays_inline: Option<Signal<bool>>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");
    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            log!("Adding image layer: {}", url);
            let options = leaflet::VideoOverlayOptions::new();
            if let Some(opacity) = opacity {
                options.set_opacity(opacity.get_untracked());
            }
            if let Some(alt) = &alt {
                options.set_alt(alt.get_untracked());
            }
            if let Some(interactive) = interactive {
                options.set_interactive(interactive.get_untracked());
            }
            if let Some(cross_origin) = &cross_origin {
                options.set_cross_origin(cross_origin.get_untracked());
            }
            if let Some(cross_origin_toggle) = cross_origin_toggle {
                options.set_cross_origin_toggle(cross_origin_toggle.get_untracked());
            }
            if let Some(error_overlay_url) = &error_overlay_url {
                options.set_error_overlay_url(error_overlay_url.get_untracked());
            }
            if let Some(z_index) = z_index {
                options.set_z_index(z_index.get_untracked());
            }
            if let Some(class_name) = &class_name {
                options.set_class_name(class_name.get_untracked());
            }
            if let Some(bubbling_mouse_events) = bubbling_mouse_events {
                options.set_bubbling_mouse_events(bubbling_mouse_events.get_untracked());
            }
            if let Some(pane) = &pane {
                options.set_pane(pane.get_untracked());
            }
            if let Some(attribution) = &attribution {
                options.set_attribution(attribution.get_untracked());
            }
            if let Some(autoplay) = autoplay {
                options.set_autoplay(autoplay.get_untracked());
            }
            if let Some(looped) = looped {
                options.set_looped(looped.get_untracked());
            }
            if let Some(keep_aspect_ratio) = keep_aspect_ratio {
                options.set_keep_aspect_ratio(keep_aspect_ratio.get_untracked());
            }
            if let Some(muted) = muted {
                options.set_muted(muted.get_untracked());
            }
            if let Some(plays_inline) = plays_inline {
                options.set_plays_inline(plays_inline.get_untracked());
            }

            let map_layer = leaflet::VideoOverlay::new_with_options(
                &url,
                &bounds.as_lat_lng_bounds(),
                &options,
            )
            .into_thread_safe_js_value();
            map_layer.add_to(&map);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
}
