use dog::Dog;
use std::ffi::CString;
use std::ops::Drop;
use std::os::raw::c_char;

struct DogWrapper {
    name: CString,
    dog: Dog,
}

impl DogWrapper {
    fn new(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        let dog = unsafe { Dog::new(name.as_ptr()) };
        DogWrapper { name, dog }
    }

    fn walk(&mut self) {
        unsafe {
            self.dog.walk();
        }
    }

    fn stop(&mut self) {
        unsafe {
            self.dog.stop();
        }
    }
}

impl Drop for DogWrapper {
    fn drop(&mut self) {
        unsafe {
            self.dog.destruct();
        }
    }
}

fn main() {
    let mut dog = DogWrapper::new("bob");
    dog.walk();
    dog.stop();
}
