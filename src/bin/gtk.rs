extern crate cairo;
extern crate gtk;
extern crate gdk;

use std::f64::consts::PI;

use gtk::prelude::*;
use gtk::DrawingArea;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let drawing_area = Box::new(DrawingArea::new)();
    window.set_title("Connect");
    window.set_default_size(800, 600);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.add(&drawing_area);

    let nb_case_height = 6. as f64;
    let nb_case_width  = 7. as f64;
    let ratio_cases_x = nb_case_width / nb_case_height;
    let ratio_cases_y = nb_case_height / nb_case_width;

    drawing_area.connect_draw(move |w, cr| {
        let width  = w.get_allocated_width() as f64;
        let height = w.get_allocated_height() as f64;
        let ratio_w = height / width;
        let ratio_h = width / height;
        cr.scale(width, height);

        let w = width / 100.;
        let h = height / 100.;

        // white background
        cr.set_source_rgb(1., 1., 1.);
        cr.rectangle(0., 0., 1., 1.);
        cr.fill();

        let mut padding_x = (1. - ratio_cases_x * ratio_w) / 2.;
        let mut padding_y =  0.05 * w / 10.;
        if height > width {
            padding_x =  0.05 * h / 10.;
            padding_y =  (1. - ratio_cases_y * ratio_h) / 2.;
        }
        let bg_width  = 1. - 2. * padding_x;
        let bg_height = 1. - 2. * padding_y;
        cr.set_source_rgb(0.5, 0.5, 1.);
        cr.rectangle(
            padding_x,
            padding_y,
            bg_width,
            bg_height
        );
        cr.fill();

        cr.set_source_rgb(0., 0., 0.);
        let bar_width  = (1. - 2. * padding_x) / nb_case_width / 10.;
        let bar_height = (1. - 2. * padding_y) / nb_case_height / 10.;
        for x in 0..(1 + nb_case_width as i32) {
            cr.rectangle(
                padding_x + x as f64 * bg_width / nb_case_width - bar_width / 2.,
                padding_y,
                bar_width,
                1. - 2. * padding_y
            );
        };
        for y in 0..(1 + nb_case_height as i32) {
            let mut translate_x = 0.;
            if y == 0 || y == nb_case_height as i32 {
                translate_x = bar_width;
            }
            cr.rectangle(
                padding_x - translate_x / 2.,
                padding_y + y as f64 * bg_height / nb_case_height - bar_height / 2.,
                1. - 2. * padding_x + translate_x,
                bar_height
            );
        };
        cr.fill();

        let line = 4.;
        let col  = 6.;
        let pos_x = (bg_width / nb_case_width - bar_width / 2.) / 2.;
        let pos_y = (bg_height / nb_case_height - bar_height / 2.) / 2.;
        cr.set_source_rgb(1.0, 0.2, 0.2);
        cr.scale(1., ratio_h);
        cr.arc(
            padding_x + (line * 2. - 1.) * (pos_x + bar_width / 4.),
            (padding_y + (col * 2. - 1.) * (pos_y + bar_height / 4.)) / ratio_h,
            pos_x * ratio_cases_y,
            0.,
            PI * 2.
        );
        cr.fill();
        Inhibit(false)
    });

    window.connect_configure_event(move |w, _| {
        let width  = w.clone().upcast::<gtk::Window>().get_size().0 - 10;
        let height = w.clone().upcast::<gtk::Window>().get_size().1 - 10;
        drawing_area.set_size_request(width, height);
        false
    });

    window.show_all();
    gtk::main();
}
