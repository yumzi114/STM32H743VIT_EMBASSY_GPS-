use core::{borrow::BorrowMut, sync::atomic::Ordering};

use atat::{asynch::AtatClient, AtatIngress, UrcChannel};
use embassy_stm32::{exti::ExtiInput, mode::Async, usart::{BufferedUartRx, UartRx, UartTx}};
use embassy_sync::pubsub::WaitResult;
use embassy_time::{Duration, Timer};
use heapless::String;
use core::fmt::Write;
use crate::{common::{self, Creg, CsmsQuery, GetManufacturerId, GetModelId, GetSoftwareVersion, GetWifiMac, Urc, AT, CSQ}, config_fn::lte_config::{c_Ingress, LTE_Client}, fmt::{info, unwrap}, APP_STATE, URC_CAPACITY, URC_SUBSCRIBERS};


type Sub_chnnel = embassy_sync::pubsub::subscriber::Subscriber<'static, embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, common::Urc, 128, 3, 1>;
#[embassy_executor::task]
pub async fn user_click(mut button:ExtiInput<'static>) {
    // let mut flag = false;
    loop {
        button.wait_for_falling_edge().await;
        let state = APP_STATE.load(Ordering::Relaxed);
        if state>=3{
            APP_STATE.store(0, Ordering::Relaxed);
        }else{
            APP_STATE.fetch_add(1, Ordering::Relaxed);
        }
        info!("CLICK");
    }
}



#[embassy_executor::task]
pub async fn lte_at_reader_task(
    mut ingress:c_Ingress,
    mut reader: BufferedUartRx<'static>,
) {
    loop {
        
        // info!("READ ");
        // CHANNEL.send(buf).await;
        ingress.read_from(&mut reader).await
        
    }
}


#[embassy_executor::task]
pub async fn lte_at_sender_task(
    mut client : LTE_Client,
) {
    let mut state: u8 = 0;
    loop {
        // These will all timeout after 1 sec, as there is no response
        if let Ok(_)=client.send(&AT).await{
            info!("IN AT");
            loop{
                
                let csq= client.send(&CSQ).await;
                let creg = client.send(&Creg).await;
                // let csms = client.send(&CsmsQuery).await;
                match (csq, creg) {
                    (Ok(csq), Ok(creg))=>{
                        info!("RSSI: {}, BER: {}", csq.rssi, csq.ber);
                        info!("STATE{:08b}", creg.stat);
                        // if let Ok(csms)=csms{
                        //     info!("CSMS s:{},Ori:{},Term:{}", csms.service, csms.mo, csms.mt);
                        // }; 
                        // if let Some(c) = creg{
                        //     info!("STATE{:08b}", c.stat);
                        // }
                        
                    }
                    _=>break
                }
                Timer::after(embassy_time::Duration::from_secs(1)).await;
            }   
        }
        
        // match state {
        //     0 => {
        //         client.send(&GetManufacturerId).await.ok();
        //     }
        //     1 => {
        //         client.send(&GetModelId).await.ok();
        //     }
        //     2 => {
        //         client.send(&GetSoftwareVersion).await.ok();
        //     }
        //     3 => {
        //         client.send(&GetWifiMac).await.ok();
        //     }
        //     _ => {},
        // }

        embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;

        // state += 1;
    }
}

#[embassy_executor::task]
pub async fn lte_at_urc_reader(
    mut sub:Sub_chnnel
) {
    loop {
        if let Some(urc) = sub.try_next_message(){
          
            
            match urc {
                WaitResult::Lagged( _ )=>{

                },
                WaitResult::Message(urc)  =>{
                    match urc {
                        Urc::MessageWaitingIndication(msg) => {
                            // let msg_str=msg.
                            info!("Waiting Indication");
                        },
                        Urc::SmsReceived(cmti)=>{
                            let msg_str = cmti.storage.as_str();
                            info!("SMS READ INDEX:{}, storege: {}",cmti.index,  msg_str);
                        }
                        _=>{

                        }
                    }
                },
            }
        }
        Timer::after(Duration::from_nanos(1)).await;
        // let urc = sub.recv().await;
        
    }   
}