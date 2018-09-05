.section ".text.boot"

.global _boot_core

_boot_core:
    // read cpu id
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
    // cpu id > 0, stop
1:  wfe
    b       1b
2:  // cpu id == 0
    // set stack
    ldr     x1, =stack_top
    add     x1, x1, #4
    mov     sp, x1

    ldr     x1, =0x3520 // 0011' 01'01' 00'10 0000
    msr     tcr_el1, x1

    /*
    MSR TTBR0_EL1, X0 // Set TTBR0
    MSR TTBR1_EL1, X1 // Set TTBR1
    MSR TCR_EL1, X2 // Set TCR
    ISB // The ISB forces these changes to be
    // seen before the MMU is enabled.
    MRS X0, SCTLR_EL1 // Read System Control Register
    // configuration data
    ORR X0, X0, #1 // Set [M] bit and enable the MMU.
    MSR SCTLR_EL1, X0 // Write System Control Register
    // configuration data
    ISB
    */
    //MRS X0, SCTLR_EL1
    //msr     hcr_el2, xzr
    //ldr     x1, =0x30C50838
    //msr     sctlr_el2, x1
    //msr     sctlr_el1, x1

    
    // should not return
    bl      reset
    b       1b
