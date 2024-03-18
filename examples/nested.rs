extern crate splines;

use std::rc::Rc;

use splines::{Interpolation, Key, Spline, interpolate::NestedSpline};

fn main() {
  let keys = vec![
    Key::new(0.0f32, 0., Interpolation::default()),
    Key::new(5.0f32, 1., Interpolation::default()),
  ];
  let spline_a = Spline::from_vec(keys);

  let keys = vec![
    Key::new(0.0f32, -10., Interpolation::default()),
    Key::new(5.0f32, 10., Interpolation::default()),
  ];
  let spline_b = Spline::from_vec(keys);

  let nested = Spline::from_vec(vec![
    Key::new(0., NestedSpline::Unit(Rc::new(spline_a)), Interpolation::default()),
    Key::new(1., NestedSpline::Unit(Rc::new(spline_b)), Interpolation::default()),
  ]);

  println!("value at 0, 0: {:?}", nested.clamped_sample(0.).unwrap().sample(0.));
  println!("value at 0, 1: {:?}", nested.clamped_sample(0.).unwrap().sample(1.));
  println!("value at 1, 0: {:?}", nested.clamped_sample(1.).unwrap().sample(0.));
  println!("value at 1, 1: {:?}", nested.clamped_sample(1.).unwrap().sample(1.));
}

