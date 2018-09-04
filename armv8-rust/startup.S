.section ".text.boot"

.global _boot_core

_boot_core:
    // read cpu id, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
    // cpu id > 0, stop
1:  wfe
    b       1b
2:  // cpu id == 0
    // set stack before our code
    ldr     x1, =stacktop
    mov     sp, x1


    // jump to Rust code, should not return
    bl      reset
    b       1b
