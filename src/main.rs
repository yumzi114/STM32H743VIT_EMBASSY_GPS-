#![no_std]
#![no_main]

mod fmt;
mod config_fn;
mod task_fn;
use core::sync::atomic::{AtomicU8, Ordering};

use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_stm32::{exti::ExtiInput, peripherals, usart::{self, Uart,Config as UConfig}};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
// use static_cell::StaticCell;

use crate::{config_fn::{display_spi_init, gps_view, lte_view, main_view, mqtt_view, sel_menu_view}, task_fn::{lte_at_reader_task, user_click}};

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;

use embassy_stm32::{bind_interrupts, gpio::{Input, Level, Output, Pull, Speed}};
use embassy_time::{Duration, Timer};
use embassy_stm32::{spi, Config};

use fmt::info;

bind_interrupts!(struct LTEIrqs {
    UART5 => usart::InterruptHandler<peripherals::UART5>;
});

static mut GLOBAL_BUFFER: [u8; 512] = [0; 512];
static APP_STATE: AtomicU8 = AtomicU8::new(0);
static CHANNEL: Channel<ThreadModeRawMutex, [u8; 8], 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(HSIPrescaler::DIV1);
        config.rcc.csi = true;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV8), // used by SPI3. 100Mhz.
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
    }
    
    // let mut app_state:App_Mod =App_Mod::MAIN;
    let p = embassy_stm32::init(config);
    let mut user_led = Output::new(p.PA1, Level::High, Speed::Low);
    let mut button: ExtiInput = ExtiInput::new(p.PB1, p.EXTI1, Pull::Down);
    let u_config = UConfig::default();
    let mut usart5 = Uart::new(p.UART5, p.PB12, p.PB13, LTEIrqs, p.DMA1_CH0, p.DMA1_CH1, u_config).unwrap();
    let (mut tx, rx) = usart5.split();
    let b_led = Output::new(p.PA0, Level::High, Speed::Low);
    // b_led.set_high();
    let (dc, rst, cs) = (p.PA3, p.PA2,p.PA4);
    let mut buffer = [0_u8; 512];
    let mut dis= display_spi_init(
        p.SPI1,
        dc.into(), 
        rst.into(), 
        cs.into(),
        p.PA5, p.PA7,p.PA6,p.DMA1_CH3,p.DMA1_CH2,
        // buffer
    ).await;
    
    spawner.must_spawn(user_click(button));
    spawner.must_spawn(lte_at_reader_task(rx));
    let mut state_flag=0;
    loop {
        // info!("Hello, World!");
        // user_led.set_high();
        // // pins_mutex.fill_screen([0xFF, 0xFF, 0xFF]).await;
        // Timer::after(Duration::from_millis(1)).await;
        // user_led.set_low();
        // pins_mutex.fill_screen([0x00, 0x00, 0x00]).await;
        // if bt
        let state_num = APP_STATE.load(Ordering::Relaxed);
        if state_num!=state_flag{
            sel_menu_view(&mut dis, state_num).await;
            match state_num {
            
                0=>{
                    main_view(&mut dis).await;
                    
                    state_flag=state_num;
                }
                1=>{
                    gps_view(&mut dis).await;
                    state_flag=state_num;
                },
                2=>{
                    lte_view(&mut dis).await;
                    state_flag=state_num;
                },
                3=>{
                    mqtt_view(&mut dis).await;
                    state_flag=state_num;
                }
                _=>{}
            }
        }
        
        Timer::after(Duration::from_nanos(1)).await;
        
    }
}








