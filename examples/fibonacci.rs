extern crate arc;

use arc::*;

fn recursive_view(value: i16) -> View {
    let (direction, color) = if value % 2 == 0 {
        if value % 4 == 0 {
            (FlexDirection::Row, 0xff_a9d2f0)
        } else {
            (FlexDirection::RowReverse, 0xff_a8ecc2)
        }
    } else {
        if (value - 1) % 4 == 0 {
            (FlexDirection::ColumnReverse, 0xff_f3c99f)
        } else {
            (FlexDirection::Column, 0xff_f5b3ac)
        }
    };

    let mut view = View::new();
    view.set_flex_grow(8.);
    view.set_background_color(color);
    view.set_margin(Edge::All, 1.5);

    let mut root = View::new();
    root.set_flex_direction(direction);
    root.add(view);

    let mut bview = if value != 0 {
        recursive_view(value - 1)
    } else {
        let mut v = View::new();
        v.set_background_color(0xff_f8e896);
        return v;
    };

    bview.set_flex_grow(5.);

    root.add(bview);
    root
}

fn main() {
    let mut c = Context::new();

    let mut window = Window::new(1280., 800.);
    window.set_title("Fibonacci-ish");
    window.set_view(recursive_view(12));
    window.set_background_color(0xff_a9b2bb);

    window.show();

    c.run();
}
