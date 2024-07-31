fn show() {
    let mut ret_addr: *mut ();
    unsafe {
        asm!(
            "mov {ptr}, [rbp + 8]",
            ptr = out(reg) ret_addr,
        );
    };
    println!("hello");
    unsafe {
        asm!(
			"mov rsp, rbp",
			"add rsp, 16",
			"mov rax, [rbp]",
			"mov rbp, rax",
			"jmp {ptr}",
			ptr = in(reg) ret_addr,
			in("rax") 0
		);
    };
    println!("hello2"); //unreachable
}

use std::arch::asm;
fn main() {
    unsafe {
		let d = 1;
        let ip = show as fn() as *const ();
        asm!(
            "call {ptr}",
            ptr = in(reg) ip,
        );
        println!("back {d}");
    }
}
