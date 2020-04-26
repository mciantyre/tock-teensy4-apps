#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <console.h>

int main(void) {
  // Repeatedly read a character from the console
  // and print a message to report it.
  while (1) {
    int c = getch();

    if (c == TOCK_FAIL) {
      printf("\ngetch() failed!\r\n");
    } else {
      printf("Got character: '%c'\r\n", (char) c);
    }
  }
}
