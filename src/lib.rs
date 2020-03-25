use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};

extern crate console_error_panic_hook;
use std::panic;

#[derive(Serialize, Deserialize)]
pub struct Record {
    Name: String,
    Type: i32,
    Value: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbData {
    Tablename: String,
    Records: Vec<Vec<Record>>,
    ColNames: Vec<String>,
    RowCount: i32,
    ColCount: i32,
    SortCol: String,
    SortDir: String,
    Offset: i32,
}

// * Helper functions, as the web_sys pieces don't seem capable of being stored in globals *
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

// DrawBarChart draws a simple bar chart, with a colour palette generated from the provided seed value
#[wasm_bindgen]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let canvas: web_sys::HtmlCanvasElement = document().get_element_by_id("barchart").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let mut width = canvas.width() as f64;
    let mut height = canvas.height() as f64;
    // {
    //     // Update the height in the global
    //     let mut h = HEIGHT.lock().unwrap();
    //     *h = height;
    // }

    // Handle window resizing
    let current_body_width = window().inner_width().unwrap().as_f64().unwrap();
    let current_body_height = window().inner_height().unwrap().as_f64().unwrap();
    if current_body_width != width || current_body_height != height {
        width = current_body_width;
        height = current_body_height;
        canvas.set_attribute("width", &width.to_string());
        canvas.set_attribute("height", &height.to_string());
    }
// canvas.set_tab_index(0); // Not sure if this is needed

// fn to_js_error(error: Error) -> JsValue {
//     JsValue::from(js_sys::Error::new(&format!("{:?}", error)))
// }

    // Get the 2D context for the canvas
    let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    // Import the data from the web page
    let data = js_sys::global();
    let data = js_sys::Reflect::get(&data, &"$scope".into()).unwrap();
    let data = js_sys::Reflect::get(&data, &"db".into()).unwrap();
    // if db.is_object() {
    //     web_sys::console::log_1(&"db: is object".into());
    // } else {
    //     web_sys::console::log_1(&"db: NOT object".into());
    // }
    // if db.is_undefined() {
    //     web_sys::console::log_1(&"db: is undefined".into());
    // } else {
    //     web_sys::console::log_1(&"db: NOT undefined".into());
    // }

    let data: DbData = data.into_serde().unwrap();
    web_sys::console::log_2(&"*** Table name ***".into(), &data.Tablename.into());
    let rows = data.Records;
    let num_rows = rows.len();

    // Count the number of items for each category
    let mut highest_val = 0;
    let item_counts: HashMap<String, i32> = HashMap::new();
    // var row js.Value
    let mut row: &Vec<Record>;
    for n in rows {
    // for (i, n) in rows.iter().enumerate() {
    // for (i, n) in 0..num_rows.iter().enumerate() {
    //     row = &rows[i];
        // row = rows.Index(i);
        // let cat_name = row[10].Value.String();
        // let cat_name = row.Index(10).Get("Value").String();
        // let item_count = strconv.Atoi(row.Index(12).Get("Value").String());
        // if err != nil {
        //     println(err)
        // }
        // let c = item_counts[cat_name];
        // item_counts[cat_name] = c + item_count;
    }

    // // Determine the highest count value, so we can automatically size the graph to fit
    // for item_count in item_counts.iter() {
    //     if item_count > highest_val {
    //         highest_val = item_count;
    //     }
    // }
    //
    // // Calculate the values used for controlling the graph positioning and display
    // let axis_caption_font_size = 20.0;
    // let axis_thickness = 5.0;
    // let border = 2.0;
    // let gap = 2.0;
    // let graph_border = 50.0;
    // let text_gap = 5.0;
    // let title_font_size = 25.0;
    // let unit_size = 3.0;
    // let x_count_font_size = 18.0;
    // let x_label_font_size = 20.0;
    // let top = border + gap;
    // let display_width = width - border - 1.0;
    // let display_height = height - border - 1.0;
    // let vert_size = highest_val * unit_size;
    // let base_line = display_height - ((display_height - vert_size) / 2.0);
    // let bar_label_y = base_line + x_label_font_size + text_gap + axis_thickness + text_gap;
    // let y_base = base_line + axis_thickness + text_gap;
    // let y_top = base_line - int(float64(vert_size)*1.2);
    // let y_length = y_base - y_top;
    //
    // // TODO: Test integrating this with DBHub
    //
    // // TODO: Calculate the graph height based upon the available size of the canvas, instead of using the current fixed unit size
    //
    // // TODO: Calculate the font sizes based upon the whether they fit in their general space
    // //       We should be able to get the font size scaling down decently, without a huge effort
    //
    // // Calculate the bar size, gap, and centering based upon the number of bars
    // let num_bars = len(item_counts);
    // let horiz_size = display_width - (graph_border * 2.0);
    // let b = float64(horiz_size) / float64(num_bars);
    // let bar_width = int(b * 0.6);
    // let bar_gap = int(b - float64(bar_width));
    // let mut bar_left= ((graph_border * 2.0) + bar_gap) / 2.0;
    // let axis_left= ((graph_border * 2.0) + bar_gap) / 2.0;
    // let axis_right= axis_left + (num_bars * bar_width) + ((num_bars - 1) * bar_gap) + axis_thickness + text_gap;
    //
    // // Calculate the y axis units of measurement
    // let (y_max, y_step) = axisMax(highest_val);
    // let y_unit= y_length / y_max;
    // let y_unit_step= y_unit * y_step;
    //
    // // TODO: Sort the categories in some useful way
    //
    // // Clear the background
    // ctx.set_fill_style(& "white");
    // ctx.fill_rect(0, 0, width, height);
    //
    // // Draw y axis marker lines
    // let y_marker_font_size = 12;
    // let y_marker_left = axis_left - axis_thickness - text_gap - 5.0;
    // ctx.set_stroke_style(&"rgb(220, 220, 220)");
    // ctx.set_fill_style(&"black".into());
    // ctx.set_font(strconv.FormatInt(int64(y_marker_font_size), 10.0)+"px serif");
    // ctx.set_text_align(&"right".into());
    // for i := float64(y_base); i >= float64(y_top); i -= float64(y_unit_step) {
    //     let marker_label = strconv.FormatInt(int64((float64(y_base)-i)/float64(y_unit)), 10.0);
    //     let marker_met = ctx.Call("measureText", marker_label);
    //     let y_marker_width = int(marker_met.Get("width").Float());
    //     ctx.begin_path();
    //     ctx.move_to(y_marker_left-y_marker_width, i);
    //     ctx.line_to(axis_right, i);
    //     ctx.stroke();
    //     ctx.fill_text(marker_label, axis_left-15.0, i-4.0);
    // }
    //
    // // Draw simple bar graph using the category data
    // let mut hue = palette as f64;
    // ctx.set_stroke_style(&"black".into());
    // ctx.set_text_align(&"center".into());
    // for (label, num) in item_counts.iter().enumerate() {
    //     let bar_height = num * unit_size;
    //     hue += goldenRatioConjugate;
    //     hue = hue % 1.0;
    //     // hue = hue - float64(int(hue)); // Simplified version of "hue % 1"
    //     ctx.set_font(&"bold "+strconv.FormatInt(int64(x_label_font_size), 10)+"px serif");
    //     ctx.set_fill_style(&hsvToRgb(hue, 0.5, 0.95));
    //     ctx.begin_path();
    //     ctx.move_to(bar_left, base_line);
    //     ctx.line_to(bar_left+bar_width, base_line);
    //     ctx.line_to(bar_left+bar_width, base_line-bar_height);
    //     ctx.line_to(bar_left, base_line-bar_height);
    //     ctx.Call("closePath");
    //     ctx.Call("fill");
    //     ctx.stroke();
    //     ctx.set_fill_style(&"black");
    //
    //     // Draw the bar label horizontally centered
    //     let text_left = float64(bar_width) / 2;
    //     ctx.fill_text(label, bar_left+int(text_left), bar_label_y);
    //
    //     // Draw the item count centered above the top of the bar
    //     ctx.set_font(&format!("{}px serif", x_count_font_size));
    //     // let s = strconv.FormatInt(int64(num), 10);
    //     text_left = float64(bar_width) / 2;
    //     ctx.fill_text(format!("{}", num), bar_left+int(text_left), base_line-bar_height-text_gap);
    //     bar_left += bar_gap + bar_width;
    // }
    //
    // // Draw axis
    // ctx.set_line_width(&axis_thickness);
    // ctx.begin_path();
    // ctx.move_to(axis_right, y_base);
    // ctx.line_to(axis_left-axis_thickness-text_gap, y_base);
    // ctx.line_to(axis_left-axis_thickness-text_gap, y_top);
    // ctx.stroke();
    //
    // // Draw title
    // let title = "Marine Litter Survey - Keep Northern Ireland Beautiful";
    // ctx.set_font(&"bold "+strconv.FormatInt(int64(title_font_size), 10.0)+"px serif");
    // ctx.set_text_align( "center");
    // let title_left = display_width / 2.0;
    // ctx.fill_text(title, title_left, top+title_font_size+20.0);
    //
    // // Draw Y axis caption
    // // Info on how to rotate text on the canvas:
    // //   https://newspaint.wordpress.com/2014/05/22/writing-rotated-text-on-a-javascript-canvas/
    // let spin_x = display_width / 2.0;
    // let spin_y = y_top + ((y_base - y_top) / 2.0) + 50.0; // TODO: Figure out why 50 works well here, then autocalculate it for other graphs
    // let y_axis_caption = "Number of items";
    // ctx.save();
    // ctx.translate(spin_x, spin_y);
    // ctx.rotate(3*Pi/2);
    // ctx.set_font(format!("italic {}px serif", axis_caption_font_size).into());
    // ctx.set_fill_style(&"black".into());
    // ctx.set_text_align(&"left".into());
    // ctx.fill_text(y_axis_caption, 0.0, -spin_x+axis_left-text_gap-int(axis_caption_font_size)-30.0); // TODO: Figure out why 30 works well here, then autocalculate it for other graphs
    // ctx.restore();
    //
    // // Draw X axis caption
    // let x_axis_caption = "Category";
    // ctx.set_font(format!("italic {}px serif", axis_caption_font_size).into());
    // let cap_left = display_width / 2.0;
    // ctx.fill_text(x_axis_caption, cap_left, bar_label_y+text_gap+axis_caption_font_size);
    //
    // // Draw a border around the graph area
    // ctx.set_line_width(2);
    // ctx.set_stroke_style(&"white".into());
    // ctx.begin_path();
    // ctx.move_to(0, 0);
    // ctx.line_to(width, 0);
    // ctx.line_to(width, height);
    // ctx.line_to(0, height);
    // ctx.close_path();
    // ctx.stroke();
    // ctx.set_line_width(2);
    // ctx.set_stroke_style(&"black".into());
    // ctx.begin_path();
    // ctx.move_to(border, border);
    // ctx.line_to(display_width, border);
    // ctx.line_to(display_width, display_height);
    // ctx.line_to(border, display_height);
    // ctx.close_path();
    // ctx.stroke();
}


// Ported from the JS here: https://martin.ankerl.com/2009/12/09/how-to-create-random-colors-programmatically/
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> String {
    let hi = h * 6.0;
    let f = h * 6.0 - hi;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let hi = hi as i32;
    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;
    if hi == 0 {
        r = v;
        g = t;
        b = p;
    }
    if hi == 1 {
        r = q;
        g = v;
        b = p;
    }
    if hi == 2 {
        r = p;
        g = v;
        b = t;
    }
    if hi == 3 {
        r = p;
        g = q;
        b = v;
    }
    if hi == 4 {
        r = t;
        g = p;
        b = v;
    }
    if hi == 5 {
        r = v;
        g = p;
        b = q;
    }

    let red = (r * 256.0) as i32;
    let green = (g * 256.0) as i32;
    let blue = (b * 256.0) as i32;
    return format!("rgb({}, {}, {})", red, green, blue);
}

// axis_max calculates the maximum value for a given axis, and the step value to use when drawing its grid lines
fn axis_max(val: i32) -> (i32, i32) {
    if val < 10 {
        return (10, 1);
    }

    // If val is less than 100, return val rounded up to the next 10
    if val < 100 {
        let x = val % 10;
        return (val + 10 - x, 10);
    }

    // If val is less than 500, return val rounded up to the next 50
    if val < 500 {
        let x = val % 50;
        return (val + 50 - x, 50);
    }
    (1000, 100)
}
