use bevy::prelude::{App, Plugin, StandardMaterial, World};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui::{CollapsingHeader, ScrollArea, Window},
    DefaultInspectorConfigPlugin,
};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_system(inspector_ui);
    }
}

fn inspector_ui(world: &mut World) {
    let egui_context = world.resource_mut::<EguiContext>().ctx_mut().clone();

    Window::new("UI").show(&egui_context, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}
