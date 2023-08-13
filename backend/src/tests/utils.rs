#[cfg(test)]
mod tests {
    use crate::utils::models::connect;

    #[test]
    fn database_connection() {
        assert!(connect().is_ok());
    }
}
