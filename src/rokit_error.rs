#[derive(Debug,Clone)]
pub struct RokitError{
    pub msg:String
} 
impl RokitError {

    pub fn new_msg(msg: String) -> Self {
        RokitError {
            msg,
        }
    }
}