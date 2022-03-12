use crate::{StructureFile, LoadFile};
use crate::prelude::*;
use crate::representations::{Representation, QueryAnyRepMut, RepresentationEnum, RepIterMut, DEFAULT_REPRESENTATIONS};
use bevy::tasks::{IoTaskPool, Task};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use futures_lite::future;
use rfd::{AsyncFileDialog, FileHandle};
use std::path::PathBuf;
use crate::camera::CamControlEvent;
#[cfg(target_arch="wasm32")]
use std::sync::RwLock;

#[derive(Default)]
pub struct MetalloproteinGuiPlugin;

impl Plugin for MetalloproteinGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(ui_system)
            .add_system(poll_file_dialog::<LoadMoleculeFileDialog>);
    }
}

fn ui_system(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    thread_pool: Res<IoTaskPool>,
    mut ev_camera: EventWriter<CamControlEvent>,
    mut q_reps: QueryAnyRepMut,
    q_structs: Query<(Entity, &StructureFile, &Children)>,
    mut q_vis: Query<&mut Visibility>,
    mut rep_window_open: Local<bool>,
    mut rep_window_entity: Local<Option<Entity>>,
    mut selected_rep: Local<RepresentationEnum>,
) {
    let ctx = egui_ctx.ctx_mut();
    egui::SidePanel::left("side_panel")
        .show(ctx, |ui| {
            for (structure_entity, structure_file, children) in q_structs.iter() {
                let heading = format!("{structure_entity:#?}:{structure_file}");
                ui.collapsing(heading, |ui| {
                    for (rep_entity, mut rep) in q_reps.iter_mut().flat_map(RepIterMut::from) {
                        if children.contains(&rep_entity) {
                            ui.horizontal(|ui| {
                                if let Ok(ref mut vis) = q_vis.get_mut(rep_entity) {
                                    ui.checkbox(&mut vis.is_visible, "");
                                }
                                ui.label(rep.name());
                                rep.representation().ui(ui);
                                if ui.button("🗑").clicked() {
                                    commands.entity(rep_entity).despawn_recursive();
                                }
                            });
                        }
                    }
                    if ui.button("+").clicked() {
                        *rep_window_entity = Some(structure_entity);
                        *rep_window_open = true;
                    }
                });
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Load").clicked() {
                        let task = thread_pool
                            .spawn(async {
                                let fd = AsyncFileDialog::new().set_directory(".").pick_file().await;
                                fd.map(LoadMoleculeFileDialog::from)
                            });
                        commands.spawn().insert(task);
                        wprintln!("Task spawned");
                    };
                    if ui.button("Center").clicked() {
                        ev_camera.send(CamControlEvent::ReCenter)
                    };
                });
            });
        });
    if *rep_window_open {
        egui::Window::new("New Representation").title_bar(false).show(ctx, |ui| {
            ui.label(format!("for {rep_window_entity:?}"));
            egui::ComboBox::from_id_source("representation_dropdown")
                .selected_text(selected_rep.name())
                .show_ui(ui, |ui| {
                    for rep in DEFAULT_REPRESENTATIONS {
                        ui.selectable_value(&mut *selected_rep, rep.clone(), rep.name());
                    }
                }
            );

            selected_rep.representation_mut().ui(ui);
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    *rep_window_open = false;
                }
                if ui.button("Add").clicked() {
                    *rep_window_open = false;
                    commands
                        .entity(rep_window_entity.expect("Orphan representation window"))
                        .with_children(|parent| {
                            match selected_rep.clone() {
                                RepresentationEnum::Licorice(inner) => parent.spawn_bundle(inner.into_bundle()),
                                RepresentationEnum::BallAndStick(inner) => parent.spawn_bundle(inner.into_bundle()),
                                RepresentationEnum::SpaceFill(inner) => parent.spawn_bundle(inner.into_bundle()),
                            };
                        });
                }
            })
        });

    }
}

#[cfg(not(target_arch = "wasm32"))]
struct LoadMoleculeFileDialog(FileHandle);

#[cfg(target_arch = "wasm32")]
struct LoadMoleculeFileDialog(RwLock<FileHandle>);

#[cfg(target_arch = "wasm32")]
impl From<FileHandle> for LoadMoleculeFileDialog{
    fn from(handle: FileHandle) -> Self {
        Self(RwLock::new(handle))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<FileHandle> for LoadMoleculeFileDialog{
    fn from(handle: FileHandle) -> Self {
        Self(handle)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Into<PathBuf> for LoadMoleculeFileDialog {
    fn into(self) -> PathBuf {
        self.0.path().into()
    }
}

fn poll_file_dialog<T: Into<PathBuf> + Send + Sync + 'static>(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut Task<Option<T>>)>,
    mut ev_loadfile: EventWriter<LoadFile>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut *task)) {
            if let Some(handle) = result {
                ev_loadfile.send(LoadFile {
                    path: handle.into(),
                });
            }
            commands.entity(entity).remove::<Task<Option<T>>>();
        }
    }
}
