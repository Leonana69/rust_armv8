OUTPUT_FORMAT("elf64-littleaarch64", "elf64-littleaarch64", "elf64-littleaarch64")
OUTPUT_ARCH(aarch64)

ENTRY(_boot_core)

SECTIONS
{
	. = 0x40000000;
	.text :
	{
        *(.text) 
	}
	.rodata :
    {
        *(.rodata .rodata.*)
    }
    .data :
    {
        *(.data .data.*)
    }
    .bss ALIGN(2):
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
	   __bss_end = .;
    }
	. = . + 0x10000;
	stack_top = .;
}
