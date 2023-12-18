use gtk::prelude::*;
use relm4::{prelude::*, drawing::DrawHandler};

const CELL_SIZE: f64 = 16.0;  // In pixels

pub(crate) struct ContentModel {
    grid: [[u8; 16]; 16],
    handler: DrawHandler,
}

pub(crate) struct ContentInit;

#[derive(Debug)]
pub(crate) enum ContentInput {
    DrawEmptyGrid,
    Paint(f64, f64),
}

#[derive(Debug)]
pub(crate) enum ContentOutput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ContentInit;

    type Input = ContentInput;
    type Output = ContentOutput;

    view! {
        gtk::Box {
            set_vexpand: true,
            set_hexpand: true,
            set_halign: gtk::Align::Center,

            // Named widgets need to have a container, so that's why I'm
            // wrapping this in a `gtk::Box`
            #[local_ref]
            area -> gtk::DrawingArea {
                set_valign: gtk::Align::Center,

                set_content_width: 256,
                set_content_height: 256,

                add_controller = gtk::GestureClick {
                    connect_pressed[sender] => move |_, _, x, y| {
                        sender.input(Self::Input::Paint(x, y));
                    },
                },

                connect_resize[sender] => move |_, _, _| {
                    sender.input(Self::Input::DrawEmptyGrid);
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            grid: [[0; 16]; 16],
            handler: DrawHandler::new(),
        };

        let area = model.handler.drawing_area();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        let cx = self.handler.get_context();

        match message {
            Self::Input::DrawEmptyGrid => (),
            Self::Input::Paint(x, y) => {
                let column = (x / CELL_SIZE) as usize;
                let line = (y / CELL_SIZE) as usize;
                self.grid[line][column] = 1;
            }
        }

        redraw(&cx, &self.grid);
    }
}

fn redraw(cx: &gtk::cairo::Context, grid: &[[u8; 16]; 16]) {
    for (i, row) in grid.iter().enumerate() {
        let y = (i as f64) * CELL_SIZE;

        for (j, &cell) in row.iter().enumerate() {
            let x = (j as f64) * CELL_SIZE;

            if cell == 1 {
                // Lit
                cx.set_source_rgb(0.0, 0.0, 0.0);  // Black
            } else if cell == 0 {
                // Unlit
                cx.set_source_rgb(1.0, 1.0, 1.0);  // White
            } else {
                unreachable!("Cell values should be either 0 or 1");
            }

            cx.rectangle(x as f64, y as f64, CELL_SIZE, CELL_SIZE);
            cx.fill()
                .expect("Should be able to fill cell");
        }
    }

    // Draw the grid
    cx.set_source_rgba(0.5, 0.5, 0.5, 0.5);  // Gray
    cx.set_line_width(1.0);
    // To draw a 1-pixel wide line, you have to aim to the "center" of the pixel
    // Otherwise, it gets blurred
    for line in 0..grid.len() {
        // Horizontal lines
        let y = (line as f64) * CELL_SIZE;
        cx.move_to(0.0, y - 0.5);  // Discount half a pixel
        cx.line_to(256.0, y - 0.5);  // Discount half a pixel
    }
    for column in 0..grid[0].len() {
        // Vertical lines
        let x = (column as f64) * CELL_SIZE;
        cx.move_to(x - 0.5, 0.0);  // Discount half a pixel
        cx.line_to(x - 0.5, 256.0);  // Discount half a pixel
    }
    cx.stroke()
        .expect("Should be able to stroke line");
}
