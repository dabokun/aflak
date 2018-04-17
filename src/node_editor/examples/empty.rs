extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

mod support;

extern crate aflak_cake as cake;
extern crate aflak_primitives as primitives;
extern crate node_editor;
use node_editor::{ComputationState, ConstantEditor, NodeEditor};

use imgui::{ImString, Ui};

const CLEAR_COLOR: [f32; 4] = [0.05, 0.05, 0.05, 1.0];

#[derive(Default)]
struct MyConstantEditor;

impl ConstantEditor<primitives::IOValue> for MyConstantEditor {
    fn editor(&self, ui: &Ui, constant: &mut primitives::IOValue) -> bool {
        use primitives::IOValue;
        match constant {
            &mut IOValue::Str(ref mut string) => {
                let mut out = ImString::with_capacity(1024);
                out.push_str(string);
                let changed = ui.input_text(im_str!("String value"), &mut out).build();
                *string = out.to_str().to_owned();
                changed
            }
            &mut IOValue::Integer(ref mut int) => {
                let mut out = *int as i32;
                let changed = ui.input_int(im_str!("Int value"), &mut out).build();
                *int = out as i64;
                changed
            }
            &mut IOValue::Float(ref mut float) => {
                ui.input_float(im_str!("Float value"), float).build()
            }
            &mut IOValue::Float2(ref mut floats) => {
                ui.input_float2(im_str!("2 floats value"), floats).build()
            }
            &mut IOValue::Float3(ref mut floats) => {
                ui.input_float3(im_str!("3 floats value"), floats).build()
            }
            _ => false,
        }
    }
}

fn main() {
    let transformations_ref = primitives::TRANSFORMATIONS.iter().collect::<Vec<_>>();
    let transformations = transformations_ref.as_slice();
    let string_constant = cake::Transformation::new_constant(primitives::IOValue::Str(
        "/home/malik/workspace/lab/aflak/data/JCMT_CO32.FITS".to_owned(),
    ));
    let mut dst = cake::DST::new();
    let a = dst.add_transform(transformations[0]);
    let _b = dst.add_transform(transformations[0]);
    let c = dst.add_transform(transformations[1]);
    let _d = dst.add_owned_transform(string_constant);
    dst.connect(cake::Output::new(a, 0), cake::Input::new(c, 0))
        .unwrap();
    dst.attach_output(cake::Output::new(c, 0)).unwrap();
    let mut node_editor = NodeEditor::from_dst(dst, transformations, MyConstantEditor);
    support::run("Node editor example".to_owned(), CLEAR_COLOR, |ui| {
        ui.window(im_str!("Node editor")).build(|| {
            node_editor.render(ui);
        });
        let outputs = node_editor.outputs();
        for output in outputs {
            let window_name = ImString::new(format!("{:?}", output));
            ui.window(&window_name).build(|| {
                let compute_state = node_editor.compute_output(&output);
                let compute_state = &*compute_state.lock().unwrap();
                match compute_state {
                    &ComputationState::NothingDone => {
                        ui.text("Initializing...");
                    }
                    &ComputationState::RunningFirstTime => {
                        ui.text("Computing...");
                    }
                    _ => match compute_state.result() {
                        Some(Err(ref e)) => {
                            ui.text(format!("{:?}", e));
                        }
                        Some(Ok(ref result)) => match result {
                            &primitives::IOValue::Str(ref string) => {
                                ui.text(format!("{:?}", string));
                            }
                            &primitives::IOValue::Integer(integer) => {
                                ui.text(format!("{:?}", integer));
                            }
                            &primitives::IOValue::Float(float) => {
                                ui.text(format!("{:?}", float));
                            }
                            &primitives::IOValue::Float2(floats) => {
                                ui.text(format!("{:?}", floats));
                            }
                            &primitives::IOValue::Float3(floats) => {
                                ui.text(format!("{:?}", floats));
                            }
                            _ => {
                                ui.text("Unimplemented");
                            }
                        },
                        None => unreachable!(),
                    },
                }
            });
        }
        true
    });
}