// min-llvm-version: 12.0.0
// needs-llvm-components: aarch64 x86
// revisions:x64 A64
// assembly-output: emit-asm
// [x64] compile-flags: --target x86_64-unknown-linux-gnu -Crelocation-model=static
// [A64] compile-flags: --target aarch64-unknown-linux-gnu -Crelocation-model=static

#![feature(no_core, lang_items)]
#![no_core]
#![crate_type="rlib"]

#[lang="sized"]
trait Sized {}

#[lang="copy"]
trait Copy {}

impl Copy for u8 {}

extern "C" {
    fn chaenomeles();
}

// CHECK-LABEL: banana:
// x64: movb   chaenomeles{{(\(%[a-z0-9]+\))?}}, %{{[a-z0-9]+}}
// A64:      adrp    [[REG:[a-z0-9]+]], chaenomeles
// A64-NEXT: ldrb    {{[a-z0-9]+}}, {{\[}}[[REG]], :lo12:chaenomeles]
#[no_mangle]
pub fn banana() -> u8 {
    unsafe {
        *(chaenomeles as *mut u8)
    }
}

// CHECK-LABEL: peach:
// x64: movb    banana{{(\(%[a-z0-9]+\))?}}, %{{[a-z0-9]+}}
// A64:      adrp    [[REG2:[a-z0-9]+]], banana
// A64-NEXT: ldrb    {{[a-z0-9]+}}, {{\[}}[[REG2]], :lo12:banana]
#[no_mangle]
pub fn peach() -> u8 {
    unsafe {
        *(banana as *mut u8)
    }
}
