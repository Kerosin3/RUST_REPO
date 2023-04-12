#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

void test_enable_device();
void test_disable_device();
void test_info_device();
void DumpHex(const void *data, size_t size); // for debugging
char* enable_termometer();

int main(int argc, char *argv[]) {
  printf("string C application\n");
  printf("--------------------------------\n");
  test_enable_device();
  test_disable_device();
  while (1) {
	test_info_device();
  	sleep(1);
  }
  return 0;
}


void test_enable_device(){
char buffer [100];
  snprintf( buffer, 100, "termometer_#0");
  char* answer = enable_termometer(1,0,buffer);
  printf("server answers: %s\n",answer);

}
void test_disable_device(){
char buffer [100];
  snprintf( buffer, 100, "termometer_#0");
  char* answer = enable_termometer(0,0,buffer);
  printf("server answers: %s\n",answer);

}
void test_info_device(){
char buffer [100];
  snprintf( buffer, 100, "termometer_#0");
  char* answer = enable_termometer(0,1,buffer);
  printf("server answers: %s\n",answer);

}

/*
 * for debugging
 * */
void DumpHex(const void *data, size_t size) {
  char ascii[17];
  size_t i, j;
  ascii[16] = '\0';
  for (i = 0; i < size; ++i) {
    printf("%02X ", ((unsigned char *)data)[i]);
    if (((unsigned char *)data)[i] >= ' ' &&
        ((unsigned char *)data)[i] <= '~') {
      ascii[i % 16] = ((unsigned char *)data)[i];
    } else {
      ascii[i % 16] = '.';
    }
    if ((i + 1) % 8 == 0 || i + 1 == size) {
      printf(" ");
      if ((i + 1) % 16 == 0) {
        printf("|  %s \n", ascii);
      } else if (i + 1 == size) {
        ascii[(i + 1) % 16] = '\0';
        if ((i + 1) % 16 <= 8) {
          printf(" ");
        }
        for (j = (i + 1) % 16; j < 16; ++j) {
          printf("   ");
        }
        printf("|  %s \n", ascii);
      }
    }
  }
}
