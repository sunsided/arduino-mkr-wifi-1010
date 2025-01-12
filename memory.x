MEMORY
{
  /* First 8KB used by Arduino bootloader */
  /*
  FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 32K
  */

  /* Overwrite the Arduino Bootloader */
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 32K
}
