use bindgen::Formatter;
use std::{env, path::PathBuf};

const HEADER_PATHS: &[&str] = &[
    "../../../quantum/quantum_keycodes.h",
    "../../../quantum/keyboard.h",
    "../../../quantum/action.h",
    "../../../drivers/oled/oled_driver.h",
    "../../../quantum/logging/sendchar.h",
    "../../../quantum/rgblight/rgblight.h",
];

fn main() {
    // bindgen will pass -D from BINDGEN_EXTRA_CLANG_ARGS to clang
    let cflags = std::env::var("BINDGEN_CFLAGS").unwrap_or_default();
    let include_directories = std::env::var("BINDGEN_INCLUDE").unwrap_or_else(|_| "-Iplatforms/chibios/converters/promicro_to_rp2040_ce -Ikeyboards/sofle/keymaps/nulls_keymap -Iusers/nulls_keymap -Ikeyboards/. -Ikeyboards/. -Ikeyboards/. -Ikeyboards/sofle -Ikeyboards/sofle/rev1 -I./platforms/chibios/boards/QMK_PM2040/configs -I. -Itmk_core -Iquantum -Iquantum/keymap_extras -Iquantum/process_keycode -Iquantum/sequencer -Idrivers -Iquantum/painter -Iquantum/unicode -Idrivers/painter/oled_panel -Idrivers/painter/sh1106 -Idrivers/painter/generic -Iplatforms/chibios/drivers/eeprom -Idrivers/eeprom -Iplatforms/chibios/drivers/wear_leveling -Idrivers/wear_leveling -Iquantum/wear_leveling -Iquantum/split_common -Idrivers/oled -Iplatforms/chibios/drivers/encoder -Idrivers/encoder -Iplatforms/chibios/drivers/vendor/RP/RP2040 -I.build/obj_sofle_rev1_nulls_keymap_elite_pi/src -Iquantum/logging -Ilib/printf/src -Ilib/printf/src/printf -Idrivers/painter/comms -Idrivers/painter/comms -Ilib/fnv -Iquantum/bootmagic/ -Iquantum/send_string/ -Itmk_core/protocol -Iplatforms -Iplatforms/chibios -Iplatforms/chibios/drivers -Itmk_core/protocol -Itmk_core/protocol/chibios -Itmk_core/protocol/chibios/lufa_utils -I./lib/chibios/os/license -I./platforms/chibios/boards/QMK_PM2040/configs -I./platforms/chibios/boards/common/configs -I./platforms/chibios/boards/QMK_PM2040/configs -I./platforms/chibios/boards/QMK_PM2040/configs -I./lib/chibios/os/common/portability/GCC -I./lib/chibios/os/common/startup/ARMCMx/compilers/GCC -I./lib/chibios/os/common/startup/ARMCMx/devices/RP2040 -I./lib/chibios/os/common/ext/ARM/CMSIS/Core/Include -I./lib/chibios/os/common/ext/RP/RP2040 -I./lib/chibios/os/rt/include -I./lib/chibios/os/common/portability/GCC -I./lib/chibios/os/common/ports/ARM-common -I./lib/chibios/os/common/ports/ARMv6-M-RP2 -I./lib/chibios/os/hal/osal/rt-nil -I./lib/chibios/os/oslib/include -I./lib/chibios/os/hal/include -I./lib/chibios/os/hal/ports/common/ARMCMx -I./lib/chibios/os/hal/ports/RP/RP2040 -I./lib/chibios/os/hal/ports/RP/LLD/DMAv1 -I./lib/chibios/os/hal/ports/RP/LLD/GPIOv1 -I./lib/chibios/os/hal/ports/RP/LLD/SPIv1 -I./lib/chibios/os/hal/ports/RP/LLD/TIMERv1 -I./lib/chibios/os/hal/ports/RP/LLD/UARTv1 -I./lib/chibios/os/hal/ports/RP/LLD/RTCv1 -I./lib/chibios/os/hal/ports/RP/LLD/WDGv1 -I./lib/chibios-contrib/os/hal/ports/RP/LLD/I2Cv1 -I./lib/chibios-contrib/os/hal/ports/RP/LLD/PWMv1 -I./lib/chibios-contrib/os/hal/ports/RP/LLD/ADCv1 -I./lib/chibios-contrib/os/hal/ports/RP/LLD/USBDv1 -I./lib/chibios/os/hal/boards/RP_PICO_RP2040 -I./lib/chibios/os/hal/lib/streams -I./lib/chibios/os/various -I. -Itmk_core -Iquantum -Iquantum/keymap_extras -Iquantum/process_keycode -Iquantum/sequencer -Idrivers -Iquantum/painter -Iquantum/unicode -Idrivers/painter/oled_panel -Idrivers/painter/sh1106 -Idrivers/painter/generic -Iplatforms/chibios/drivers/eeprom -Idrivers/eeprom -Iplatforms/chibios/drivers/wear_leveling -Idrivers/wear_leveling -Iquantum/wear_leveling -Iquantum/split_common -Idrivers/oled -Iplatforms/chibios/drivers/encoder -Idrivers/encoder -Iplatforms/chibios/drivers/vendor/RP/RP2040 -I./lib/chibios//os/various/pico_bindings/dumb/include -I./lib/pico-sdk/src/common/pico_base/include -I./lib/pico-sdk/src/rp2_common/pico_platform/include -I./lib/pico-sdk/src/rp2_common/hardware_base/include -I./lib/pico-sdk/src/rp2_common/hardware_clocks/include -I./lib/pico-sdk/src/rp2_common/hardware_claim/include -I./lib/pico-sdk/src/rp2_common/hardware_flash/include -I./lib/pico-sdk/src/rp2_common/hardware_gpio/include -I./lib/pico-sdk/src/rp2_common/hardware_irq/include -I./lib/pico-sdk/src/rp2_common/hardware_pll/include -I./lib/pico-sdk/src/rp2_common/hardware_pio/include -I./lib/pico-sdk/src/rp2_common/hardware_sync/include -I./lib/pico-sdk/src/rp2_common/hardware_timer/include -I./lib/pico-sdk/src/rp2_common/hardware_resets/include -I./lib/pico-sdk/src/rp2_common/hardware_watchdog/include -I./lib/pico-sdk/src/rp2_common/hardware_xosc/include -I./lib/pico-sdk/src/rp2040/hardware_regs/include -I./lib/pico-sdk/src/rp2040/hardware_structs/include -I./lib/pico-sdk/src/boards/include -I./lib/pico-sdk/src/rp2_common/pico_bootrom/include -Iplatforms/chibios/vendors/RP -I./lib/pico-sdk/src/common/pico_base/include -I./lib/pico-sdk/src/rp2_common/pico_platfrom/include -I./lib/pico-sdk/src/rp2_common/hardware_divider/include -Ikeyboards/. -Ikeyboards/. -Ikeyboards/. -Ikeyboards/sofle -Ikeyboards/sofle/rev1 -I./platforms/chibios/boards/QMK_PM2040/configs".to_string());
    let include_directories = include_directories
        .split(" ")
        .map(|s| format!("-I../../../{}", &s[2..]))
        .collect::<Vec<String>>();

    let extra_clang_args = cflags
        .split(" ")
        .filter(|s| s.starts_with("-D"))
        .collect::<Vec<&str>>()
        .join(" ");

    unsafe {
        env::set_var("BINDGEN_EXTRA_CLANG_ARGS", extra_clang_args);
    }

    let bindings = bindgen::builder()
        .headers(HEADER_PATHS.iter().map(|path| path.to_string()))
        .use_core()
        .clang_args(HEADER_PATHS.iter().map(|path| {
            format!("-I{}", {
                let mut split = path.split("/").collect::<Vec<_>>();
                split.pop();
                split.join("/")
            })
        }))
        .clang_args(include_directories)
        .clang_arg("-I/usr/lib/picolibc/riscv64-unknown-elf/include")
        .clang_arg("-D NULLPTR_BINDGEN")
        .clang_arg("-D MATRIX_ROWS=10")
        .clang_arg("-D MATRIX_COLS=7")
        .clang_arg("-D RGB_MATRIX_LED_COUNT=35")
        .clang_arg("-D EEPROM_TEST_HARNESS")
        .formatter(Formatter::Rustfmt)
        .rustified_enum(".*")
        .constified_enum_module(".*")
        .generate_comments(true)
        .header_contents("progmem.h", "#define PROGMEM")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
