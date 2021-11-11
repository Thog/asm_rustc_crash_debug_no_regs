#![no_std]
#![feature(asm)]

#[derive(Debug)]
#[repr(C)]
pub struct ThreadContext {
    pub R: [u32; 16]
}

//#[naked]
#[no_mangle]
pub unsafe extern "C" fn arm_create_thread_context() -> ThreadContext {
    let r0: u32;
    let r1: u32;
    let r2: u32;
    let r3: u32;
    let r4: u32;
    let r5: u32;
    let r6: u32;
    let r7: u32;
    let r8: u32;
    let r9: u32;
    let r10: u32;
    let r11: u32;
    let r12: u32;
    let r13: u32;
    let r14: u32;
    let r15: u32;

    asm!(
        "mov {0}, r0",
        "mov {1}, r1",
        "mov {2}, r2",
        "mov {3}, r3",
        "mov {4}, r4",
        "mov {5}, r5",
        "mov {6}, r6",
        "mov {7}, r7",
        "mov {8}, r8",
        "mov {9}, r9",
        "mov {10}, r10",
        "mov {11}, r11",
        "mov {12}, r12",
        "mov {13}, r13",
        "mov {14}, r14",
        "mov {15}, r15",
        out(reg) r0,
        out(reg) r1,
        out(reg) r2,
        out(reg) r3,
        out(reg) r4,
        out(reg) r5,
        out(reg) r6,
        out(reg) r7,
        out(reg) r8,
        out(reg) r9,
        out(reg) r10,
        out(reg) r11,
        out(reg) r12,
        out(reg) r13,
        out(reg) r14,
        out(reg) r15,
    );
    
    ThreadContext {
        R: [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15]
    }
}
