#include <stdio.h>
#include <stdlib.h>

char *hello_rust (char*);

int main(int argc, char **argv)
{
   char *resp = hello_rust(argv[1]);
   printf("%s", resp);
   free(resp);

   return 0;
}

