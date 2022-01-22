#include <stdio.h>
#include <stdlib.h>
#include "polychat_plugin.h"

Account create_account(void) {
    int *account = (int *) malloc(sizeof(int));
    *account = 42;
    return (Account) account;
}

void print(Account account) {
    printf("Hello %d!\n",  *((int*) account));
}

void destroy_account(Account account) {
    free(account);
}