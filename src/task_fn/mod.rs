use core::sync::atomic::Ordering;

use atat::{asynch::AtatClient, AtatIngress};
use embassy_stm32::{exti::ExtiInput, mode::Async, usart::{BufferedUartRx, UartRx, UartTx}};
use embassy_time::Timer;
use heapless::String;
use core::fmt::Write;
use crate::{common::{self, Creg, GetManufacturerId, GetModelId, GetSoftwareVersion, GetWifiMac, AT, CSQ}, config_fn::lte_config::{c_Ingress, LTE_Client}, fmt::{info, unwrap}, APP_STATE};



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
                match (csq, creg) {
                    (Ok(csq), Ok(creg))=>{
                        info!("RSSI: {}, BER: {}", csq.rssi, csq.ber);
                        info!("STATE{:08b}", creg.stat);
                        Timer::after(embassy_time::Duration::from_secs(1)).await;
                    }
                    
                    _=>break
                }
                // Timer::after(embassy_time::Duration::from_secs(1)).await;
            }   
        }
        
        embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;

        // state += 1;
    }
}