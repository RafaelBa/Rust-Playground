mod csv_reader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::csv_reader::get_file_content().len(), 6952);
    }
}
