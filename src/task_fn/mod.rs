use core::sync::atomic::Ordering;

use embassy_stm32::{exti::ExtiInput, mode::Async, usart::UartRx};
use embassy_time::Timer;

use crate::{fmt::info, APP_STATE};



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
pub async fn lte_at_reader_task(mut rx: UartRx<'static, Async>) {

    loop {
        info!("task B");
        Timer::after_secs(1).await;
    }
}