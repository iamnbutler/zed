use crate::WebRenderer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct QuadDemo {
    renderer: WebRenderer,
}

#[wasm_bindgen]
impl QuadDemo {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<QuadDemo, JsValue> {
        console_error_panic_hook::set_once();

        Ok(QuadDemo {
            renderer: WebRenderer::new(canvas_id)?,
        })
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        self.renderer.resize(width, height)
    }

    #[wasm_bindgen]
    pub fn draw_simple_quad(&mut self) -> Result<(), JsValue> {
        self.renderer.clear();

        // Blue quad at (50, 50) with size 200x150
        self.renderer.draw_quad(
            50.0, 50.0, // x, y
            200.0, 150.0, // width, height
            79, 195, 247, 255, // rgba
        )
    }

    #[wasm_bindgen]
    pub fn draw_rounded_quad(&mut self) -> Result<(), JsValue> {
        self.renderer.clear();

        // Purple rounded quad
        self.renderer.draw_rounded_quad(
            300.0, 50.0, // x, y
            200.0, 150.0, // width, height
            156, 39, 176, 255,  // rgba
            20.0, // corner radius
        )
    }

    #[wasm_bindgen]
    pub fn draw_bordered_quad(&mut self) -> Result<(), JsValue> {
        self.renderer.clear();

        // Dark gray quad with orange border
        self.renderer.draw_bordered_quad(
            50.0, 250.0, // x, y
            200.0, 150.0, // width, height
            45, 45, 45, 255, // fill rgba
            255, 152, 0, 255, // border rgba
            3.0, // border width
        )
    }

    #[wasm_bindgen]
    pub fn draw_complex_scene(&mut self) -> Result<(), JsValue> {
        self.renderer.clear();

        // Background quad with dashed border
        self.renderer.draw_bordered_rounded_quad(
            300.0, 250.0, // x, y
            200.0, 150.0, // width, height
            33, 33, 33, 255, // fill rgba
            100, 100, 100, 255,  // border rgba
            2.0,  // border width
            10.0, // corner radius
            true, // dashed
        )?;

        // Inner semi-transparent quad
        self.renderer.draw_rounded_quad(
            320.0, 270.0, // x, y
            160.0, 110.0, // width, height
            76, 175, 80, 200, // rgba with transparency
            5.0, // corner radius
        )
    }

    #[wasm_bindgen]
    pub fn draw_multiple_quads(&mut self) -> Result<(), JsValue> {
        self.renderer.clear();

        // Draw a grid of quads
        for row in 0..3 {
            for col in 0..4 {
                let x = 50.0 + (col as f32 * 180.0);
                let y = 50.0 + (row as f32 * 180.0);

                let r = (255.0 * (col as f32 / 3.0)) as u8;
                let g = (255.0 * (row as f32 / 2.0)) as u8;
                let b = 128;

                if (row + col) % 2 == 0 {
                    self.renderer
                        .draw_rounded_quad(x, y, 150.0, 150.0, r, g, b, 220, 10.0)?;
                } else {
                    self.renderer.draw_bordered_quad(
                        x, y, 150.0, 150.0, r, g, b, 220, 255, 255, 255, 128, 2.0,
                    )?;
                }
            }
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn draw_nested_quads(&mut self) -> Result<(), JsValue> {
        self.renderer.clear();

        // Draw nested quads with decreasing opacity
        for i in 0..5 {
            let offset = i as f32 * 30.0;
            let size = 400.0 - (i as f32 * 60.0);
            let opacity = 255 - (i * 40);

            self.renderer.draw_rounded_quad(
                100.0 + offset,
                100.0 + offset,
                size,
                size,
                79,
                195,
                247,
                opacity as u8,
                20.0 - (i as f32 * 3.0),
            )?;
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.renderer.clear();
    }
}
