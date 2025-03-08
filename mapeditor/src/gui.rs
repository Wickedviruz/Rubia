use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::world::{BlockType, WorldMap};

#[derive(Resource)]
pub struct CurrentTool {
    pub block_type: BlockType,
}

impl Default for CurrentTool {
    fn default() -> Self {
        Self {
            block_type: BlockType::Grass,
        }
    }
}

pub fn gui_system(
    mut contexts: EguiContexts,
    mut current_tool: ResMut<CurrentTool>,
    mut world_map: ResMut<WorldMap>,
) {
    egui::Window::new("Rubia Map Editor").show(contexts.ctx_mut(), |ui| {
        ui.label("Välj blocktyp:");
        ui.horizontal(|ui| {
            if ui.button("Grass").clicked() {
                current_tool.block_type = BlockType::Grass;
            }
            if ui.button("Dirt").clicked() {
                current_tool.block_type = BlockType::Dirt;
            }
            if ui.button("Stone").clicked() {
                current_tool.block_type = BlockType::Stone;
            }
        });

        if ui.button("Save Map").clicked() {
            crate::io::save_map(&world_map, "data/map.rubia").unwrap();
        }
        if ui.button("Load Map").clicked() {
            *world_map = crate::io::load_map("data/map.rubia").unwrap();
        }
    });
}
