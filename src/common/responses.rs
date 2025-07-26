use atat::atat_derive::AtatResp;
use atat::heapless::String;

/// 4.1 Manufacturer identification
/// Text string identifying the manufacturer.
#[derive(Clone, Debug, AtatResp)]
pub struct ManufacturerId {
    pub id: String<64>,
}

/// Model identification
/// Text string identifying the manufacturer.
#[derive(Clone, Debug, AtatResp)]
pub struct ModelId {
    pub id: String<64>,
}

/// Software version identification
/// Read a text string that identifies the software version of the module.
#[derive(Clone, Debug, AtatResp)]
pub struct SoftwareVersion {
    pub id: String<64>,
}

/// 7.11 Wi-Fi Access point station list +UWAPSTALIST
#[derive(Clone, AtatResp)]
pub struct WifiMac {
    pub mac_addr: atat::heapless_bytes::Bytes<12>,
}
#[derive(Clone, AtatResp)]
pub struct CSQResponse {
    #[at_arg(position = 0)]
    pub rssi: u8,

    #[at_arg(position = 1)]
    pub ber: u8,
}


#[derive(Clone, AtatResp)]
pub struct CregResponse {
    #[at_arg(position = 0)]
    pub n: u8,

    #[at_arg(position = 1)]
    pub stat: u8,

    #[at_arg(position = 2)]
    pub lac: Option<heapless::String<8>>,

    #[at_arg(position = 3)]
    pub ci: Option<heapless::String<8>>,

    #[at_arg(position = 4)]
    pub act: Option<u8>,
}


#[derive(Clone, AtatResp)]
pub struct CsmsResponse {
    #[at_arg(position = 0)]
    pub service: u8, // 0 = no support, 1 = supported
    #[at_arg(position = 1)]
    pub mo: u8,      // Mobile Originated SMS
    #[at_arg(position = 2)]
    pub mt: u8,      // Mobile Terminated SMS
}


