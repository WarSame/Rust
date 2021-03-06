mod cacher;

#[cfg(test)]
mod tests {
    use cacher::Cacher;

    #[test]
    fn repeated_runs_same() {
        let mut cacher = Cacher::new(|x| x);
        let run1 = cacher.value(5);
        let run2 = cacher.value(7);

        assert_ne!(run1, run2);
    }
}