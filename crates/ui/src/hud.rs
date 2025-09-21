use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hud_ui);
    }
}

fn hud_ui(mut egui_ctx: EguiContexts) {
    let ctx = egui_ctx.ctx_mut();
    egui::Area::new("hud_placeholder")
        .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -10.0])
        .show(ctx, |ui| {
            ui.label("HUD placeholder (health, ammo, etc.)");
        });
}
