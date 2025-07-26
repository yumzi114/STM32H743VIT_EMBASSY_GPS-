use atat::atat_derive::AtatResp;

#[derive(Clone, AtatResp)]
pub struct MessageWaitingIndication;

#[derive(Clone, AtatResp)]
pub struct CmtiUrc {
    #[at_arg(position = 0)]
    pub storage: heapless::String<4>,
    #[at_arg(position = 1)]
    pub index: u32,
}