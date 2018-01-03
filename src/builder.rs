#[cfg(test)]
mod tests {
    #[test]
    fn it_builds_a_valid_puzzle() {
        let builder = Builder {};
        let puzzle = builder.build_complete_puzzle();

        assert!(valid(puzzle));
    }
}
