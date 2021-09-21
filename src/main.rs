#![no_std]  // 標準ライブラリを無効化する
#![no_main]  // コンパイラに通常のエントリポイント(main)を使いたくないことを伝える

use core::panic::PanicInfo;

// OSのエントリポイントを上書き
#[no_mangle] // no_mangle attribute ... 名前修飾(コンパイラが関数名をユニークにするために色々付け足すこと）を無効化する
pub extern "C" fn _start() -> ! {   // Cの呼び出し規約を使用するように指定(ほとんどのシステムのデフォルトエントポイント)
    loop{}
}

// この関数はPanic時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // この関数は戻り値を返すべきでない->never型を返す
    loop {} 
}
