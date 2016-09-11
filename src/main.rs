fn main() {
  let mut vec1 = Vec::new();
  vec1.push(1);
  vec1.push(2);
  take_ownership_as_immutable(vec1);
  //vec1.push(4); // <-- fails
  
  let mut vec2 = Vec::new();
  vec2.push(1);
  vec2.push(2);
  take_full_ownership(vec2);
  //vec2.push(4); // <-- fails

  let mut vec3 = Vec::new();
  vec3.push(1);
  vec3.push(2);
  borrow_immutable(&vec3);
  vec3.push(4);
  println!("fn main says vec3 = {:?}", vec3);

  borrow_mutable(&mut vec3);
  vec3.push(99);
  println!("fn main says vec3 = {:?}", vec3);
}

// passage of ownership, no mutability
fn take_ownership_as_immutable(vect: Vec<i32>) {
  //vect.push(3); // <-- fails
  println!("fn take_ownership_as_immutable says vect = {:?}", vect);
}

// passage of ownership + allow mutability
fn take_full_ownership(mut vect: Vec<i32>) {
  vect.push(3);
  println!("fn take_full_ownership says vect = {:?}", vect);
}

// borrowing, no mutability
fn borrow_immutable(vect: &Vec<i32>) {
  println!("fn borrow_immutable says vect = {:?}", vect);
}

// borrowing + mutability
fn borrow_mutable(vect: &mut Vec<i32>) {
  vect.push(3);
  println!("fn borrow_mutable says vect = {:?}", vect);
}

// silly comment to verify commits from the Vagrant
