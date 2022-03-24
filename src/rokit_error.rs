pub struct RokitError{
    pub msg:String,
    pub ignore:bool
} 
impl Default for RokitError {
    fn default() -> Self {
        RokitError {
            msg: "".to_string(),
            ignore: false
        }
    }
}
impl RokitError {

    pub fn new_msg(msg: String) -> Self {
        RokitError {
            msg,
            ..Default::default()
        }
    }
    pub fn new(ignore: bool, msg:String) -> Self {
        RokitError {
            msg,
            ignore
        }
    }
}