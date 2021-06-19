#include <stdlib.h>
#include <stdio.h>

int main () {
  int a = 3;
  int b = 4;
  int r;
  while (1) {
    printf("3 + 4 = ");
    scanf("%d", &r);
    if (r == 7) {
      printf("Ok\n");
      return 0;
    }
    printf("Wrong\n");
  }
  return 0;
}
