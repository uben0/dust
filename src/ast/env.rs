use std::collections::HashMap;

pub trait Env<T> {
    fn get(&self, name: &str) -> Option<&T>;
    fn set(&mut self, name: String, value: T);
    fn get_mut(&mut self, name: &str) -> Option<&mut T>;
}

pub struct EnvFrame<'a, T> {
    pred: Option<&'a mut dyn Env<T>>,
    frame: HashMap<String, T>,
}
impl<'a, T> EnvFrame<'a, T> {
    pub fn new(pred: Option<&'a mut dyn Env<T>>) -> Self {
        Self{
            pred,
            frame: HashMap::new(),
        }
    }
}
impl<'a, T> Env<T> for EnvFrame<'a, T> {
    fn set(&mut self, name: String, value: T) {
        self.frame.insert(name, value);
    }
    fn get(&self, name: &str) -> Option<&T> {
        self.frame.get(name).or_else(|| self.pred.as_ref().and_then(|p| p.get(name)))
    }
    fn get_mut(&mut self, name: &str) -> Option<&mut T> {
        match self.frame.get_mut(name) {
            Some(value) => Some(value),
            None => self.pred.as_mut().and_then(|p| p.get_mut(name)),
        }
    }
}