use crate::config::Config;

use super::draw_area;
use gtk::{prelude::*, Application, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4_layer_shell::{Layer, LayerShell};

pub fn new_window(app: &Application, mut config: Config) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(app);
    let size = config.size;

    window.init_layer_shell();

    window.set_layer(Layer::Top);

    // Push other windows out of the way
    // window.auto_exclusive_zone_enable();

    window.set_anchor(config.edge, true);
    if let Some(pos) = config.position {
        window.set_anchor(pos, true);
    }

    draw_area::setup_draw(
        &window,
        config.edge,
        size,
        config.event_map.take().unwrap(),
        config.color,
        config.extra_trigger_size,
        config.transition_duration,
        config.frame_rate,
    );

    window.connect_show(move |w: &gtk::ApplicationWindow| {
        // transparency background !! may not work for some gtk4 theme, and idk what to do with it !!
        let provider = CssProvider::new();
        provider
            // .load_from_string("window.background { background: unset; border: 1px solid white; }");
            .load_from_string("window.background { background: unset; }");
        gtk::style_context_add_provider_for_display(
            &WidgetExt::display(w),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    window.present();
    window
}
