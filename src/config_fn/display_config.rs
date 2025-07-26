use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb666, prelude::{Point, Primitive, RgbColor, Size, WebColors}, primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle}, text::Text};
use mipidsi::{interface::SpiInterface, options::{ColorOrder, Orientation, Rotation}, Display}; 
use embassy_stm32::{ gpio::{AnyPin, Level, Output, Speed}, mode::Async, peripherals::{DMA1_CH2, DMA1_CH3,  PA5, PA6, PA7}, spi::{self, Spi}, time::mhz};
use embassy_time::{Delay, Duration};

use embedded_hal_bus::spi::ExclusiveDevice;
use mipidsi::{models::ILI9486Rgb666, Builder};
use profont::{PROFONT_14_POINT, PROFONT_18_POINT, PROFONT_24_POINT};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use crate::GLOBAL_BUFFER;
type Spi_Display=Display<SpiInterface<'static, ExclusiveDevice<Spi<'static, Async>, Output<'static>, embedded_hal_bus::spi::NoDelay>, Output<'static>>, ILI9486Rgb666, Output<'static>>;
pub enum App_Mod{
    MAIN,
    LTE,
    GPS,
    MQTT
}
pub async fn display_spi_init<'a>(
    spi:embassy_stm32::peripherals::SPI1,
    dc: AnyPin,
    rst:AnyPin,
    cs:AnyPin, 
    // spi :Spi<'static, Async>, 
    d_sck:PA5,
    d_mosi:PA7,
    d_miso: PA6,
    tx_dma:DMA1_CH3,
    rx_dma:DMA1_CH2,
    // buffer: &'a mut [u8; 512],
)->Spi_Display
{
    let cs = Output::new(cs, Level::High, Speed::Low);
    let dc = Output::new(dc, Level::High, Speed::Low);
    let rst = Output::new(rst, Level::High, Speed::Low);
    let mut spi_config = spi::Config::default();
    spi_config.frequency = mhz(100);
    let mut delay = Delay;
    let spi: Spi<Async> = spi::Spi::new(
        spi,
        d_sck,
        d_mosi,
        d_miso,
        tx_dma,
        rx_dma,
        spi_config
    );
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let buffer: &mut [u8; 512] = unsafe { &mut GLOBAL_BUFFER };
    let di = SpiInterface::new(spi_device, dc, buffer);
 
    let mut display= Builder::new(ILI9486Rgb666, di)
        .color_order(ColorOrder::Bgr)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();
    let orientation = Orientation {
        rotation: Rotation::Deg0,
        mirrored: true,
    };
    display.set_orientation(orientation).unwrap();
    display.clear(Rgb666::BLACK).unwrap();
    let style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb666::WHITE);
    let de_text_style = MonoTextStyle::new(&PROFONT_18_POINT,Rgb666::WHITE );
    Text::new("EOMi ~", Point::new(12, 24), style)
            .draw(&mut display)
            .unwrap();
    Line::new(Point::new(1, 40), Point::new(350, 40))
        .into_styled(PrimitiveStyle::with_stroke(Rgb666::CSS_RED, 10))
        .draw(&mut display).unwrap();

    
    Line::new(Point::new(1, 100), Point::new(350, 100))
        .into_styled(PrimitiveStyle::with_stroke(Rgb666::CSS_RED, 10))
        .draw(&mut display).unwrap();
    Text::new("MAIN", Point::new(10, 75), de_text_style).draw(&mut display).unwrap();
    Text::new("GPS", Point::new(95, 75), de_text_style).draw(&mut display).unwrap();
    Text::new("LTE", Point::new(185, 75), de_text_style).draw(&mut display).unwrap();
    Text::new("MQTT", Point::new(260, 75), de_text_style).draw(&mut display).unwrap();
    main_view(&mut display).await;
    display
   
}

pub async fn main_view(display: &mut Spi_Display){
    let content_head = MonoTextStyle::new(&PROFONT_18_POINT, Rgb666::WHITE);

    let r_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb666::BLACK) 
            .build();
    // let color = if *state {Rgb666::GREEN}else{Rgb666::WHITE};
    
    Rectangle::new(Point::new(10, 125), Size::new(160, 20))
        .into_styled(r_style)
        .draw(display)
        .unwrap();
    Text::new("MAIN", Point::new(12, 140), content_head)
        .draw( display)
        .unwrap();
}
pub async fn gps_view(display: &mut Spi_Display){
    let content_head = MonoTextStyle::new(&PROFONT_18_POINT, Rgb666::WHITE);
    let r_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb666::BLACK) 
            .build();
    Rectangle::new(Point::new(10, 125), Size::new(160, 20))
        .into_styled(r_style)
        .draw(display)
        .unwrap();
    Text::new("GPS", Point::new(12, 140), content_head)
        .draw( display)
        .unwrap();
}
pub async fn lte_view(display: &mut Spi_Display){
    let content_head = MonoTextStyle::new(&PROFONT_18_POINT, Rgb666::WHITE);
    let r_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb666::BLACK) 
            .build();
    Rectangle::new(Point::new(10, 125), Size::new(160, 20))
        .into_styled(r_style)
        .draw(display)
        .unwrap();
    Text::new("LTE", Point::new(12, 140), content_head)
        .draw( display)
        .unwrap();
}
pub async fn mqtt_view(display: &mut Spi_Display){
    let content_head = MonoTextStyle::new(&PROFONT_18_POINT, Rgb666::WHITE);
    let r_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb666::BLACK) 
            .build();
    Rectangle::new(Point::new(10, 125), Size::new(160, 20))
        .into_styled(r_style)
        .draw(display)
        .unwrap();
    Text::new("MQTT", Point::new(12, 140), content_head)
        .draw( display)
        .unwrap();
}

pub async fn sel_menu_view(display: &mut Spi_Display,sel_num:u8){
    let sel_text_style = MonoTextStyle::new(&PROFONT_18_POINT,Rgb666::CSS_GREEN_YELLOW );
    let de_text_style = MonoTextStyle::new(&PROFONT_18_POINT,Rgb666::WHITE );
    let color = match  sel_num{
        0=>{
            Text::new("MAIN", Point::new(10, 75), sel_text_style).draw(display).unwrap();
            Text::new("GPS", Point::new(95, 75), de_text_style).draw(display).unwrap();
            Text::new("LTE", Point::new(185, 75), de_text_style).draw( display).unwrap();
            Text::new("MQTT", Point::new(260, 75), de_text_style).draw(display).unwrap();
        },
        1=>{
            Text::new("MAIN", Point::new(10, 75), de_text_style).draw(display).unwrap();
            Text::new("GPS", Point::new(95, 75), sel_text_style).draw(display).unwrap();
            Text::new("LTE", Point::new(185, 75), de_text_style).draw( display).unwrap();
            Text::new("MQTT", Point::new(260, 75), de_text_style).draw(display).unwrap();
        },
        2=>{
            Text::new("MAIN", Point::new(10, 75), de_text_style).draw(display).unwrap();
            Text::new("GPS", Point::new(95, 75), de_text_style).draw(display).unwrap();
            Text::new("LTE", Point::new(185, 75), sel_text_style).draw( display).unwrap();
            Text::new("MQTT", Point::new(260, 75), de_text_style).draw(display).unwrap();
        },
        3=>{
            Text::new("MAIN", Point::new(10, 75), de_text_style).draw(display).unwrap();
            Text::new("GPS", Point::new(95, 75), de_text_style).draw(display).unwrap();
            Text::new("LTE", Point::new(185, 75), de_text_style).draw( display).unwrap();
            Text::new("MQTT", Point::new(260, 75), sel_text_style).draw(display).unwrap();
        },
        _=>{

        }        
    };
    
    
    
}