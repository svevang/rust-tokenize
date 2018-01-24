#include <stdio.h>

void *hello_rust (char*);

int main(int argc, char **argv)
{
   printf("%s", hello_rust(argv[1]));
   return 0;
}

