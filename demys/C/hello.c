#include <stdlib.h>
#include <unistd.h>

int main (int argc, char* argv[]) {
  if (argc > 1 ) {
    printf("(%d)Hellooooooooooooo world: 0:%s, 1:%s\n", getpid(), argv[0], argv[1]);
  } else {
    printf("(%d)Hellooooooooooooo world\n", getpid());
  }
  return 0;
}
