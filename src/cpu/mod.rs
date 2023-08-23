pub struct CPU{


    pub(crate) memory: Vec<u8>
}
pub fn init() ->CPU{
    return CPU{
        memory: vec![0; 4096]
    };
}

impl CPU{

}