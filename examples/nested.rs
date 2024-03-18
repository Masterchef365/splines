extern crate splines;

use std::rc::Rc;

use splines::{Interpolation, Key, Spline, interpolate::NestedSpline};

fn main() {
  let interp = Interpolation::Linear;


  let keys = vec![
    Key::new(0.0f32, 0.0f32, interp),
    Key::new(1.0f32, 1.0f32, interp),
  ];
  let spline_a = Spline::from_vec(keys);

  let keys = vec![
    Key::new(0.0f32, -10.0f32, interp),
    Key::new(1.0f32, 10.0f32, interp),
  ];
  let spline_b = Spline::from_vec(keys);

  let nested_a = Spline::from_vec(vec![
    Key::new(0.0f32, NestedSpline::Unit(Rc::new(spline_a)), Interpolation::Linear),
    Key::new(1.0f32, NestedSpline::Unit(Rc::new(spline_b)), Interpolation::Linear),
  ]);



  let keys = vec![
    Key::new(0.0f32, 0.0f32, interp),
    Key::new(1.0f32, 1.0f32, interp),
  ];
  let spline_a = Spline::from_vec(keys);

  let keys = vec![
    Key::new(0.0f32, -100.0f32, interp),
    Key::new(1.0f32, 100.0f32, interp),
  ];
  let spline_b = Spline::from_vec(keys);

  let nested_b = Spline::from_vec(vec![
    Key::new(0.0f32, NestedSpline::Unit(Rc::new(spline_a)), Interpolation::Linear),
    Key::new(1.0f32, NestedSpline::Unit(Rc::new(spline_b)), Interpolation::Linear),
  ]);



  let nested = Spline::from_vec(vec![
    Key::new(0.0f32, NestedSpline::Unit(Rc::new(nested_a)), Interpolation::Linear),
    Key::new(1.0f32, NestedSpline::Unit(Rc::new(nested_b)), Interpolation::Linear),
  ]);

  dbg!(std::any::type_name_of_val(&nested));

  println!("value at 0, 0, 0: {:?}", nested.sample(0.1).unwrap().sample(0.1).unwrap().sample(0.1));
  println!("value at 0, 0, 1: {:?}", nested.sample(0.1).unwrap().sample(0.1).unwrap().sample(0.9));
  println!("value at 0, 1, 0: {:?}", nested.sample(0.1).unwrap().sample(0.9).unwrap().sample(0.1));
  println!("value at 0, 1, 1: {:?}", nested.sample(0.1).unwrap().sample(0.9).unwrap().sample(0.9));

  println!("value at 1, 0, 0: {:?}", nested.sample(0.9).unwrap().sample(0.1).unwrap().sample(0.1));
  println!("value at 1, 0, 1: {:?}", nested.sample(0.9).unwrap().sample(0.1).unwrap().sample(0.9));
  println!("value at 1, 1, 0: {:?}", nested.sample(0.9).unwrap().sample(0.9).unwrap().sample(0.1));
  println!("value at 1, 1, 1: {:?}", nested.sample(0.9).unwrap().sample(0.9).unwrap().sample(0.9));

}

