
use piston_window;
/**
 * A Single Slideshow Element
 */
pub trait SlideshowElement {
  fn render(&self);
  fn update(&self, updtateArgs: piston_window::UpdateArgs);
  fn init(&self);

}
