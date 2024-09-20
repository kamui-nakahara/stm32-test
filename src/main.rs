#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // パニックをキャッチするために`rust_begin_unwind`にブレークポイントを置くことができます
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // ITM 経由でメッセージをログに記録します。ITM サポートが必要です。
                     // use panic_semihosting as _; // メッセージをホストのstderrに記録します。デバッガが必要です。

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4::stm32f446::{interrupt, Interrupt, NVIC};

#[entry]
fn main() -> ! {
    asm::nop(); // リリースモードでメインの最適化を中止しないようにするには、コードを追加するときに削除します。

    loop {
        // your code goes here
    }
}
