#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void hello() {
  printf("Hello World: ");
}

void illegal() {
  printf("Illegal statement");
}


int main (int argc, char* argv[]) {
  if (argc > 1 ) {
    hello();
    printf("(%d): 0:%s, 1:%s\n", getpid(), argv[0], argv[1]);
  } else {
    hello();
    printf("(%d)\n", getpid());
  }
  return 0;
}
