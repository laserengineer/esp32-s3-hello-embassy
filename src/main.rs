#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU8, Ordering};
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{GpioPin, Input, Io, Level, Output, Pull},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::timg::TimerGroup,
};

// extern crate alloc;
use core::mem::MaybeUninit;

static LED_STATUS: AtomicU8 = AtomicU8::new(0);

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[embassy_executor::task]
async fn print_hello() {
    let run_symbol = ['*', '&', '%', '$'];
    loop {
        for symbol in run_symbol {
            let now = Instant::now();
            esp_println::println!(
                "{}_Application on the embassy framework! {}",
                now.as_millis(),
                symbol
            );
            Timer::after(Duration::from_millis(1_000)).await;
        }
    }
}

#[embassy_executor::task]
async fn led_control(mut led: Output<'static, GpioPin<1>>) {
    loop {
        let led_status = LED_STATUS.load(Ordering::Relaxed);
        match led_status {
            0 => {
                led.toggle();
            }
            1 => {
                led.set_high();
            }
            2 => {
                led.set_low();
            }
            _ => {}
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn detect_switch(mut switch: Input<'static, GpioPin<2>>) {
    // polling approach
    // let mut last_switch_state  = true;
    // loop
    // {
    //     if switch.is_high() &  (last_switch_state == false){
    //         esp_println::println!("button pressed");
    //         let mut led_status = LED_STATUS.load(Ordering::Relaxed);
    //         led_status += 1;
    //         if led_status >2 {led_status = 0;}
    //         LED_STATUS.store(led_status, Ordering::Relaxed);
    //         esp_println::println!("INFO: led status: {}", led_status);
    //     }
    //     last_switch_state = switch.is_high();
    //     Timer::after(Duration::from_millis(100)).await;
    // }

    // async approach (require adding "async" feature in esp_hal crate)
    loop {
        switch.wait_for_rising_edge().await;
        esp_println::println!("button pressed");
        let mut led_status = LED_STATUS.load(Ordering::Relaxed);
        led_status += 1;
        if led_status > 2 {
            led_status = 0;
        }
        LED_STATUS.store(led_status, Ordering::Relaxed);
        esp_println::println!("INFO: led status: {}", led_status);
        Timer::after(Duration::from_millis(50)).await;

        switch.wait_for_low().await;
        Timer::after(Duration::from_millis(50)).await;
    }
}

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = Output::new(io.pins.gpio1, Level::High);
    let switch = Input::new(io.pins.gpio2, Pull::Down);
    // let mut last_switch_state  = true;

    init_heap();

    esp_println::logger::init_logger_from_env();
    esp_hal_embassy::init(&clocks, timg0);

    spawner.spawn(print_hello()).ok();
    spawner.spawn(detect_switch(switch)).ok();
    spawner.spawn(led_control(led)).ok();

    // let run_symbol = ['*', '&', '%', '$'];
    loop {
        Timer::after(Duration::from_millis(1_000)).await;
        // for symbol in run_symbol {
        //     esp_println::println!("Application on the embassy framework! {}", symbol);
        //     Timer::after(Duration::from_millis(3_000)).await;
        // }
    }
}
