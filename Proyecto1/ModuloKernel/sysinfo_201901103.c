#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/sched.h>
#include <linux/sched/signal.h>
#include <linux/mm.h>
#include <linux/slab.h>
#include <linux/vmstat.h>
#include <linux/sysinfo.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Dani");
MODULE_DESCRIPTION("Módulo del kernel para capturar métricas de memoria y contenedores en JSON");
MODULE_VERSION("1.2");

#define PROC_FILENAME "sysinfo_201901103"
#define MAX_CMDLINE_LENGTH 256
#define CONTAINER_ID_LENGTH 64
//#define CONTAINER_PREFIX "contenedores-"

// Función para obtener el porcentaje de memoria utilizada con enteros escalados
static unsigned long get_mem_percentage(struct task_struct *task) {
    struct mm_struct *mm = task->mm;
    unsigned long total_mem = totalram_pages() << (PAGE_SHIFT - 10); // Total RAM en KB
    unsigned long rss = mm ? get_mm_rss(mm) << (PAGE_SHIFT - 10) : 0; // RSS en KB

    // Escalar el resultado multiplicándolo por 1000 para evitar decimales
    return total_mem ? ((rss * 1000) / total_mem) : 0;
}

// Función para obtener la línea de comandos de un proceso y retornar un apuntador a la cadena
static char *get_process_cmdline(struct task_struct *task) {

    /* 
        Creamos una estructura mm_struct para obtener la información de memoria
        Creamos un apuntador char para la línea de comandos
        Creamos un apuntador char para recorrer la línea de comandos
        Creamos variables para guardar las direcciones de inicio y fin de los argumentos y el entorno
        Creamos variables para recorrer la línea de comandos
    */
    struct mm_struct *mm;
    char *cmdline, *p;
    unsigned long arg_start, arg_end, env_start;
    int i, len;


    // Reservamos memoria para la línea de comandos
    cmdline = kmalloc(MAX_CMDLINE_LENGTH, GFP_KERNEL);
    if (!cmdline)
        return NULL;

    // Obtenemos la información de memoria
    mm = get_task_mm(task);
    if (!mm) {
        kfree(cmdline);
        return NULL;
    }

    /* 
       1. Primero obtenemos el bloqueo de lectura de la estructura mm_struct para una lectura segura
       2. Obtenemos las direcciones de inicio y fin de los argumentos y el entorno
       3. Liberamos el bloqueo de lectura de la estructura mm_struct
    */
    down_read(&mm->mmap_lock);
    arg_start = mm->arg_start;
    arg_end = mm->arg_end;
    env_start = mm->env_start;
    up_read(&mm->mmap_lock);

    // Obtenemos la longitud de la línea de comandos y validamos que no sea mayor a MAX_CMDLINE_LENGTH - 1
    len = arg_end - arg_start;

    if (len > MAX_CMDLINE_LENGTH - 1)
        len = MAX_CMDLINE_LENGTH - 1;

    // Obtenemos la línea de comandos de  la memoria virtual del proceso
    /* 
        Por qué de la memoria virtual del proceso?
        La memoria virtual es la memoria que un proceso puede direccionar, es decir, la memoria que un proceso puede acceder
    */
    if (access_process_vm(task, arg_start, cmdline, len, 0) != len) {
        mmput(mm);
        kfree(cmdline);
        return NULL;
    }

    // Agregamos un caracter nulo al final de la línea de comandos
    cmdline[len] = '\0';

    // Reemplazar caracteres nulos por espacios
    p = cmdline;
    for (i = 0; i < len; i++)
        if (p[i] == '\0')
            p[i] = ' ';

    // Liberamos la estructura mm_struct
    mmput(mm);
    return cmdline;
}

// Función que muestra la información
static int sysinfo_show(struct seq_file *m, void *v) {
    struct task_struct *task;
    struct sysinfo i;
    unsigned long task_cpu_time;
    unsigned long total_mem, free_mem, used_mem;
    int first = 1; // Para saber si es el primer proceso

    // Obtener información del sistema
    si_meminfo(&i);

    // Calcular memoria total y libre en KB
    total_mem = i.totalram << (PAGE_SHIFT - 10); // Total RAM en KB
    free_mem = i.freeram << (PAGE_SHIFT - 10);   // Memoria libre en KB
    used_mem = total_mem - free_mem;             // Memoria utilizada

    // Imprimir información de la memoria
    seq_printf(m, "{\n");
    seq_printf(m, "  \"ram\": {\n");
    seq_printf(m, "    \"total\": %lu ,\n", total_mem);
    seq_printf(m, "    \"libre\": %lu ,\n", free_mem);
    seq_printf(m, "    \"uso\": %lu \n", used_mem);
    seq_printf(m, "  },\n");
    seq_printf(m, "  \"containers\": [\n");

    // Recorrer todos los procesos en el sistema
    for_each_process(task) {
        if (strstr(task->comm, "python")) {  // Filtro básico para procesos de contenedores
            struct mm_struct *mm = task->mm;
            unsigned long vm_size = 0, vm_rss = 0;
            unsigned long mem_percentage = 0;
            unsigned long cpu_usage_scaled = 0;
            char *cmdline = NULL;

            // Calcular el uso de CPU para este proceso usando enteros
            task_cpu_time = (task->utime + task->stime);
            cpu_usage_scaled = (1000 * task_cpu_time) / jiffies;
            cmdline = get_process_cmdline(task);

            // Calcular el uso de memoria para este proceso
            if (mm) {
                vm_size = mm->total_vm << (PAGE_SHIFT - 10); // VSZ en KB
                vm_rss = get_mm_rss(mm) << (PAGE_SHIFT - 10); // RSS en KB
                mem_percentage = get_mem_percentage(task); // Porcentaje de memoria escalado
            }

            if (!first) {
                seq_printf(m, "    },\n"); // Cierra el proceso anterior con coma
            }
            first = 0; // A partir del primer proceso ya no es el primero
            
            // Imprimir información del proceso con enteros escalados
            seq_printf(m, "    {\n");
            seq_printf(m, "      \"pid\": \"%d\",\n", task->pid);
            seq_printf(m, "      \"nombre\": \"%s\",\n", task->comm);
            seq_printf(m, "      \"cmdline\": \"%s\",\n", cmdline ? cmdline : "N/A");
            seq_printf(m, "      \"vsz\": %lu ,\n", vm_size);
            seq_printf(m, "      \"rss\": %lu ,\n", vm_rss);
            seq_printf(m, "      \"memoria_usage\": %lu.%03lu,\n", mem_percentage / 1000, mem_percentage % 1000); // Eliminar el signo de porcentaje
            seq_printf(m, "      \"cpu_usage\": %lu.%03lu\n", cpu_usage_scaled / 1000, cpu_usage_scaled % 1000); // Eliminar el signo de porcentaje

            if(cmdline){
                kfree(cmdline);
            }
        }
    }

    // Cerrar el último proceso sin coma
    if (!first) {
        seq_printf(m, "    }\n"); // Cerrar sin coma
    }

    seq_printf(m, "  ]\n");
    seq_printf(m, "}\n");

    return 0;
}

// Función para abrir el archivo en /proc
static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

// Operaciones del archivo en /proc
static const struct proc_ops sysinfo_fops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release,
};

// Función de inicialización del módulo
static int __init sysinfo_init(void) {
    proc_create(PROC_FILENAME, 0, NULL, &sysinfo_fops);
    printk(KERN_INFO "sysinfo module loaded\n");
    return 0;
}

// Función de limpieza del módulo
static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_FILENAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);