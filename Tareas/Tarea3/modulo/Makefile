obj-m += sysinfo_201901103.o

# Agrega las banderas necesarias
CFLAGS_sysinfo_201901103.o := -msse

all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean