//! The renderer cannot be a normal system because it holds values that must be used on the main
//! thread. It cannot be executed in parallel like other systems. Another complication is that it
//! returns a `Result` whereas normal systems do not return anything.

#![allow(dead_code, unused_variables, unused_imports)] //TODO(EX#2): remove this line

use specs::{SystemData, ReadStorage, Join, World, prelude::ResourceId};
use sdl2::{
    rect::{Point, Rect},
    render::{WindowCanvas, Texture},
};

use crate::components::{BoundingBox, Sprite};

/// Data from the world required by the renderer
#[derive(SystemData)]
pub struct RendererData<'a> {
    bounding_boxes: ReadStorage<'a, BoundingBox>,
    sprites: ReadStorage<'a, Sprite>,
}

impl<'a> RendererData<'a> {
    pub fn render(&self, canvas: &mut WindowCanvas, textures: &[Texture]) -> Result<(), String> {
        let RendererData {bounding_boxes, sprites} = self;

        //TODO(EX#2): Copy the code from the render() function of goal.rs, player.rs, or enemy.rs
        // and then adapt it to work in this function
        for (&BoundingBox(bounds), &Sprite {texture_id, region: sprite_rect}) in (bounding_boxes, sprites).join() {
            let (width, height) = canvas.output_size()?;
            let screen_pos = bounds.center() + Point::new((width/2) as i32, (height/2) as i32);
            let screen_rect = Rect::from_center(screen_pos, sprite_rect.width() as u32, sprite_rect.height() as u32);

            // Copy the current frame onto the canvas
            canvas.copy(&textures[texture_id], sprite_rect, screen_rect)?;

            //TODO(EX#2): Figure out how to render given the bounding box, texture_id, and
            // sprite_rect. HINT: How do you determine the position based on the bounding box?
            // Go to the specs documentation and look up `Rect`.
        }


        Ok(())
    }
}
