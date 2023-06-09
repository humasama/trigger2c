#include "default_header.h"

#define MAX_SYSCALL_NUM 128

static syscall_fn calls[MAX_SYSCALL_NUM + 1];
static int32_t nr_call;

int register_syscall(uint32_t cmd, syscall_fn func)
{
    if (nr_call >= MAX_SYSCALL_NUM)
        return -1;

    calls[cmd] = func;
    nr_call++;
    return 0;
}

void trigger(uint32_t cmd)
{
    if (calls[cmd])
        calls[cmd]();
    else
        printf("syscall %d is not null\n", cmd);
}
