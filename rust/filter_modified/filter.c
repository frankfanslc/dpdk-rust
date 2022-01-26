#include <stdio.h>
#include "filter.h"

int main(int argc, char* argv[]){

        uint8_t addr1[6] = {0x90, 0xe2, 0xba, 0xb1, 0x2c, 0x62};
        uint8_t addr2[6] = {0x90, 0xe2, 0xba, 0xb1, 0x2c, 0x76};
        bool a;

        printf("%d\n", atoi(argv[1]));

        if (atoi(argv[1]) == 1){
                a = ip_add(addr1);
                printf("1111\n");
        }else{
                a = ip_add(addr2);
                printf("2222\n");
        }

        if(a == true){
                printf("success\n");
        }else
                printf("failed\n");
        return 0;
}
