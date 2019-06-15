extern crate gtk;
extern crate relm;

use gtk::Orientation::Vertical;
use gtk::{ButtonExt, Inhibit, LabelExt, OrientableExt, WidgetExt};
use relm::connect;
use relm::connect_stream;
use relm::{timeout, Relm, Widget};
use relm_attributes::widget;
use relm_derive::Msg;

mod data_model;

// Define the structure of the model.
pub struct Model {
    counter: i32,
}

// The messages that can be sent to the update function.
#[derive(Msg)]
pub enum Msg {
    Decrement,
    Increment,
    Quit,
    Show,
}
use self::Msg::*;

#[widget]
impl Widget for Win {
    // The initial model.
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        timeout(relm.stream(), 1000, || Show);
        Model { counter: 0 }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            Decrement => self.model.counter -= 1,
            Increment => self.model.counter += 1,
            Quit => gtk::main_quit(),
            Show => self.dec_button.set_visible(true),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                // Set the orientation property of the Box.
                orientation: Vertical,
                // Create a Button inside the Box.
                #[name="hello"]
                gtk::Label {
                    // Bind the text property of the label to the counter attribute of the model.
                    text: "This is Sparda",
                },
                gtk::Button {
                    // Send the message Increment when the button is clicked.
                    clicked => Increment,
                    // TODO: check if using two events of the same name work.
                    label: "+",
                },
                #[name="label"]
                gtk::Label {
                    // Bind the text property of the label to the counter attribute of the model.
                    text: &self.model.counter.to_string(),
                },
                #[name="dec_button"]
                gtk::Button {
                    clicked => Decrement,
                    label: "-",
                    visible: false,
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
