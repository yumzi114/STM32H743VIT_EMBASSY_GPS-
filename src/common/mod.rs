mod responses;
mod urc;

use atat::atat_derive::AtatUrc;
use atat::atat_derive::{AtatCmd, AtatResp};

use crate::common::responses::{CSQResponse, CregResponse, ManufacturerId, ModelId, SoftwareVersion, WifiMac};

#[derive(Clone, AtatResp)]
pub struct NoResponse;

#[derive(Clone, AtatCmd)]
#[at_cmd("", NoResponse, timeout_ms = 1000)]
pub struct AT;

#[derive(Clone, AtatUrc)]
pub enum Urc {
    #[at_urc("+UMWI")]
    MessageWaitingIndication(urc::MessageWaitingIndication),
}


#[derive(Clone, AtatCmd)]
#[at_cmd("+CGMI", ManufacturerId)]
pub struct GetManufacturerId;

/// Model identification +CGMM
///
/// Read a text string that identifies the device model.
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGMM", ModelId)]
pub struct GetModelId;

/// Software version identification +CGMR
///
/// Read a text string that identifies the software version of the module
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGMR", SoftwareVersion)]
pub struct GetSoftwareVersion;

/// 7.12 Wi-Fi MAC address +UWAPMACADDR
///
/// Lists the currently used MAC address.
#[derive(Clone, AtatCmd)]
#[at_cmd("+UWAPMACADDR", WifiMac)]
pub struct GetWifiMac;

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSQ", CSQResponse)]
pub struct CSQ;

#[derive(Clone, AtatCmd)]
#[at_cmd("+CREG?", CregResponse)]
pub struct Creg;