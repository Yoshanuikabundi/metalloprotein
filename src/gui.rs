use crate::StructureFile;
use crate::prelude::*;
use crate::LoadFile;
use bevy::tasks::{IoTaskPool, Task};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use futures_lite::future;
use rfd::AsyncFileDialog;
use rfd::FileHandle;
use std::path::PathBuf;
use crate::camera::CamControlEvent;
use crate::representations::{BorrowAllRepLists, BorrowAllRepListsTupleWithEntity};
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
    q_reps: Query<BorrowAllRepListsTupleWithEntity>,
    q_structs: Query<&StructureFile>
) {
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .max_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            for tuple in q_reps.iter() {
                let entity = tuple.0;
                let heading = if let Ok(file) = q_structs.get(entity) {
                    if let Some(name) = file.0.file_name() {
                        format!("{name:?}")
                    } else {
                        format!("{entity:?}")
                    }
                } else {
                        format!("{entity:?}")
                };
                ui.collapsing(heading, |ui| {
                    let tuple = BorrowAllRepLists::from(tuple);
                    tuple.for_each_iter(|reps| {
                        for rep in reps {
                            ui.label(format!("{rep:?}"));
                        }
                    })
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
                    if ui.button("Remove").clicked() {};
                });
            });
        });
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
