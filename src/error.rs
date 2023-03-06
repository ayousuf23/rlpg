pub trait RlpgErr {
    fn get_err_message(&self) -> String;
}