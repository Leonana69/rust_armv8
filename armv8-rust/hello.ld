MEMORY
{
  FLASH (rx) : ORIGIN = 0x40000000, LENGTH = 128K
}

SECTIONS
{
	. = 0x10000000;
	.text :
	{
	*(.text)
	KEEP(*(isr_vector))  
	} > FLASH
	.rodata :
    {
        *(.rodata .rodata.*)
    }
    .data :
    {
        *(.data .data.*)
    }
    .bss ALIGN(8):
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
	__bss_end = .;
    }
	. = . + 0x10000;
	stacktop = .;
}
