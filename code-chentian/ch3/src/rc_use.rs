#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn use_rc() {
        let a = Rc::new(1);
        let b = a.clone();
        let c = a.clone();
    }
}