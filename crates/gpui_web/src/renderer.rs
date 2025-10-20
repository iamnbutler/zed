use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Web Canvas renderer for GPUI primitives
#[wasm_bindgen]
pub struct WebRenderer {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    device_pixel_ratio: f64,
}

#[wasm_bindgen]
impl WebRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WebRenderer, JsValue> {
        let window = web_sys::window().ok_or("no window")?;
        let document = window.document().ok_or("no document")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or(format!("canvas '{}' not found", canvas_id))?
            .dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .ok_or("failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let device_pixel_ratio = window.device_pixel_ratio();

        Ok(Self {
            canvas,
            context,
            device_pixel_ratio,
        })
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        // Set canvas size accounting for device pixel ratio
        let scaled_width = (width as f64 * self.device_pixel_ratio) as u32;
        let scaled_height = (height as f64 * self.device_pixel_ratio) as u32;

        self.canvas.set_width(scaled_width);
        self.canvas.set_height(scaled_height);

        // Set CSS size
        let style = self.canvas.style();
        style.set_property("width", &format!("{}px", width))?;
        style.set_property("height", &format!("{}px", height))?;

        // Scale context for high DPI
        self.context
            .scale(self.device_pixel_ratio, self.device_pixel_ratio)?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
    }

    #[wasm_bindgen]
    pub fn draw_quad(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Result<(), JsValue> {
        self.context.save();

        // Simple rectangle path
        self.context.begin_path();
        self.context
            .rect(x as f64, y as f64, width as f64, height as f64);

        // Fill with color
        let color = format!("rgba({},{},{},{})", r, g, b, a as f64 / 255.0);
        self.context.set_fill_style(&JsValue::from_str(&color));
        self.context.fill();

        self.context.restore();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn draw_rounded_quad(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        corner_radius: f32,
    ) -> Result<(), JsValue> {
        self.context.save();

        let x = x as f64;
        let y = y as f64;
        let width = width as f64;
        let height = height as f64;
        let radius = corner_radius as f64;

        // Create rounded rectangle path
        self.context.begin_path();
        self.context.move_to(x + radius, y);
        self.context.line_to(x + width - radius, y);
        self.context
            .arc_to(x + width, y, x + width, y + radius, radius);
        self.context.line_to(x + width, y + height - radius);
        self.context.arc_to(
            x + width,
            y + height,
            x + width - radius,
            y + height,
            radius,
        );
        self.context.line_to(x + radius, y + height);
        self.context
            .arc_to(x, y + height, x, y + height - radius, radius);
        self.context.line_to(x, y + radius);
        self.context.arc_to(x, y, x + radius, y, radius);
        self.context.close_path();

        // Fill with color
        let color = format!("rgba({},{},{},{})", r, g, b, a as f64 / 255.0);
        self.context.set_fill_style(&JsValue::from_str(&color));
        self.context.fill();

        self.context.restore();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn draw_bordered_quad(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        fill_r: u8,
        fill_g: u8,
        fill_b: u8,
        fill_a: u8,
        border_r: u8,
        border_g: u8,
        border_b: u8,
        border_a: u8,
        border_width: f32,
    ) -> Result<(), JsValue> {
        self.context.save();

        // Draw rectangle
        self.context.begin_path();
        self.context
            .rect(x as f64, y as f64, width as f64, height as f64);

        // Fill
        if fill_a > 0 {
            let fill_color = format!(
                "rgba({},{},{},{})",
                fill_r,
                fill_g,
                fill_b,
                fill_a as f64 / 255.0
            );
            self.context.set_fill_style(&JsValue::from_str(&fill_color));
            self.context.fill();
        }

        // Stroke border
        if border_a > 0 && border_width > 0.0 {
            let border_color = format!(
                "rgba({},{},{},{})",
                border_r,
                border_g,
                border_b,
                border_a as f64 / 255.0
            );
            self.context
                .set_stroke_style(&JsValue::from_str(&border_color));
            self.context.set_line_width(border_width as f64);
            self.context.stroke();
        }

        self.context.restore();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn draw_bordered_rounded_quad(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        fill_r: u8,
        fill_g: u8,
        fill_b: u8,
        fill_a: u8,
        border_r: u8,
        border_g: u8,
        border_b: u8,
        border_a: u8,
        border_width: f32,
        corner_radius: f32,
        dashed: bool,
    ) -> Result<(), JsValue> {
        self.context.save();

        let x = x as f64;
        let y = y as f64;
        let width = width as f64;
        let height = height as f64;
        let radius = corner_radius as f64;

        // Create rounded rectangle path
        self.context.begin_path();
        self.context.move_to(x + radius, y);
        self.context.line_to(x + width - radius, y);
        self.context
            .arc_to(x + width, y, x + width, y + radius, radius);
        self.context.line_to(x + width, y + height - radius);
        self.context.arc_to(
            x + width,
            y + height,
            x + width - radius,
            y + height,
            radius,
        );
        self.context.line_to(x + radius, y + height);
        self.context
            .arc_to(x, y + height, x, y + height - radius, radius);
        self.context.line_to(x, y + radius);
        self.context.arc_to(x, y, x + radius, y, radius);
        self.context.close_path();

        // Fill
        if fill_a > 0 {
            let fill_color = format!(
                "rgba({},{},{},{})",
                fill_r,
                fill_g,
                fill_b,
                fill_a as f64 / 255.0
            );
            self.context.set_fill_style(&JsValue::from_str(&fill_color));
            self.context.fill();
        }

        // Stroke border
        if border_a > 0 && border_width > 0.0 {
            let border_color = format!(
                "rgba({},{},{},{})",
                border_r,
                border_g,
                border_b,
                border_a as f64 / 255.0
            );
            self.context
                .set_stroke_style(&JsValue::from_str(&border_color));
            self.context.set_line_width(border_width as f64);

            if dashed {
                let dashes = js_sys::Array::new();
                dashes.push(&JsValue::from(5.0));
                dashes.push(&JsValue::from(5.0));
                self.context.set_line_dash(&dashes)?;
            }

            self.context.stroke();

            if dashed {
                self.context.set_line_dash(&js_sys::Array::new())?;
            }
        }

        self.context.restore();
        Ok(())
    }

    #[wasm_bindgen]
    pub fn save_state(&mut self) {
        self.context.save();
    }

    #[wasm_bindgen]
    pub fn restore_state(&mut self) {
        self.context.restore();
    }

    #[wasm_bindgen]
    pub fn set_clip_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.context.begin_path();
        self.context
            .rect(x as f64, y as f64, width as f64, height as f64);
        self.context.clip();
    }

    #[wasm_bindgen]
    pub fn translate(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        self.context.translate(x as f64, y as f64)
    }

    #[wasm_bindgen]
    pub fn scale(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        self.context.scale(x as f64, y as f64)
    }
}
