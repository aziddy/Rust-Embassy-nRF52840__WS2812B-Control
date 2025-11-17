MEMORY
{
  /* Reserve space for Adafruit bootloader (starts at 0x26000) */
  /*FLASH : ORIGIN = 0x00026000, LENGTH = 848K
  RAM : ORIGIN = 0x20000008, LENGTH = 255K */

  /* No bootloader - start from beginning of flash */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}