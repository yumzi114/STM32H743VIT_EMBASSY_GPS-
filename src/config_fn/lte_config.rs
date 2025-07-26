use atat::{asynch::Client, Ingress, ResponseSlot, UrcChannel};
use embassy_stm32::{bind_interrupts, peripherals::{self, PB12, PB13, UART5}, usart::{self, BufferedInterruptHandler, BufferedUart, BufferedUartRx, BufferedUartTx, Config as UConfig}};
use static_cell::StaticCell;
use atat::DefaultDigester;
use crate::common::Urc;


const INGRESS_BUF_SIZE: usize = 1024;
const URC_CAPACITY: usize = 128;
const URC_SUBSCRIBERS: usize = 3;

pub type c_Ingress = Ingress<'static, DefaultDigester<Urc>, Urc, INGRESS_BUF_SIZE, URC_CAPACITY, URC_SUBSCRIBERS>;
pub type LTE_Client = Client<'static, BufferedUartTx<'static>, INGRESS_BUF_SIZE>;
static INGRESS_BUF: StaticCell<[u8; INGRESS_BUF_SIZE]> = StaticCell::new();
static TX_BUF: StaticCell<[u8; 16]> = StaticCell::new();
static RX_BUF: StaticCell<[u8; 16]> = StaticCell::new();
static RES_SLOT: ResponseSlot<INGRESS_BUF_SIZE> = ResponseSlot::new();
static URC_CHANNEL: UrcChannel<Urc, URC_CAPACITY, URC_SUBSCRIBERS> = UrcChannel::new();
static BUF: StaticCell<[u8; 1024]> = StaticCell::new();
bind_interrupts!(struct LTEIrqs {
    UART5 => BufferedInterruptHandler<UART5>;
});
// bind_interrupts!(struct LTEIrqs {
//     UART5 => usart::InterruptHandler<peripherals::UART5>;
// });

pub async fn lte_init(uart:UART5, tx:PB13, rx:PB12)
->(
    LTE_Client,
    c_Ingress,
    BufferedUartRx<'static>,
)
{
    let u_config = UConfig::default();
    let uart = BufferedUart::new(
        uart,
        LTEIrqs,
        rx,
        tx,
        TX_BUF.init([0; 16]),
        RX_BUF.init([0; 16]),
        u_config,
    ).unwrap();
    let (writer, reader) = uart.split();
    static RES_SLOT: ResponseSlot<INGRESS_BUF_SIZE> = ResponseSlot::new();
    let ingress: Ingress<DefaultDigester<Urc>, Urc, INGRESS_BUF_SIZE, URC_CAPACITY, URC_SUBSCRIBERS> = Ingress::new(
        DefaultDigester::<Urc>::default(),
        INGRESS_BUF.init([0; INGRESS_BUF_SIZE]),
        &RES_SLOT,
        &URC_CHANNEL,
    );
    let client = Client::new(
        writer,
        &RES_SLOT,
        BUF.init([0; 1024]),
        atat::Config::default(),
    );
    (client, ingress,reader)
}