#include <stdio.h>
#include <stdint.h>

typedef int32_t (*syscall_fn)(void);

int register_syscall(uint32_t cmd, syscall_fn callback);
void trigger(uint32_t cmd);
