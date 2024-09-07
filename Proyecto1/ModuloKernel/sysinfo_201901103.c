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
//#define CONTAINER_PREFIX "contenedores-"

// Función para obtener el porcentaje de memoria utilizada con enteros escalados
static unsigned long get_mem_percentage(struct task_struct *task) {
    struct mm_struct *mm = task->mm;
    unsigned long total_mem = totalram_pages() << (PAGE_SHIFT - 10); // Total RAM en KB
    unsigned long rss = mm ? get_mm_rss(mm) << (PAGE_SHIFT - 10) : 0; // RSS en KB

    // Escalar el resultado multiplicándolo por 1000 para evitar decimales
    return total_mem ? ((rss * 1000) / total_mem) : 0;
}

// Función que muestra la información
static int sysinfo_show(struct seq_file *m, void *v) {
    struct task_struct *task;
    struct sysinfo i;
    unsigned long task_cpu_time;
    //unsigned long cpu_usage;
    unsigned long total_mem, free_mem, used_mem;

    // Obtener información del sistema
    si_meminfo(&i);

    // Calcular memoria total y libre en KB
    total_mem = i.totalram << (PAGE_SHIFT - 10); // Total RAM en KB
    free_mem = i.freeram << (PAGE_SHIFT - 10);   // Memoria libre en KB
    used_mem = total_mem - free_mem;             // Memoria utilizada

    // Imprimir información de la memoria
    seq_printf(m, "{\n");
    seq_printf(m, "  \"ram\": {\n");
    seq_printf(m, "    \"total\": %lu KB,\n", total_mem);
    seq_printf(m, "    \"libre\": %lu KB,\n", free_mem);
    seq_printf(m, "    \"uso\": %lu KB\n", used_mem);
    seq_printf(m, "  },\n");
    seq_printf(m, "  \"containers\": [\n");

    // Recorrer todos los procesos en el sistema
    for_each_process(task) {
        if (strstr(task->comm, "python")) {  // Filtro básico para procesos de contenedores
        //if (strncmp(task->comm, CONTAINER_PREFIX, strlen(CONTAINER_PREFIX)) == 0) {
            struct mm_struct *mm = task->mm;
            unsigned long vm_size = 0, vm_rss = 0;
            unsigned long mem_percentage = 0;
            unsigned long cpu_usage_scaled = 0;

            // Calcular el uso de CPU para este proceso usando enteros
            task_cpu_time = (task->utime + task->stime);
            cpu_usage_scaled = (1000 * task_cpu_time) / jiffies;

            // Calcular el uso de memoria para este proceso
            if (mm) {
                vm_size = mm->total_vm << (PAGE_SHIFT - 10); // VSZ en KB
                vm_rss = get_mm_rss(mm) << (PAGE_SHIFT - 10); // RSS en KB
                mem_percentage = get_mem_percentage(task); // Porcentaje de memoria escalado
            }

            // Imprimir información del proceso con enteros escalados
            seq_printf(m, "    {\n");
            seq_printf(m, "      \"pid\": \"%d\",\n", task->pid);
            seq_printf(m, "      \"nombre\": \"%s\",\n", task->comm);
            seq_printf(m, "      \"idContainer\": \"%s\",\n", task->comm); // Usar nombre del proceso como ID del contenedor
            seq_printf(m, "      \"Vsz\": %lu KB,\n", vm_size);
            seq_printf(m, "      \"Rss\": %lu KB,\n", vm_rss);
            seq_printf(m, "      \"memoria_usage\": \"%lu.%03lu%%\",\n", mem_percentage / 1000, mem_percentage % 1000); // Mostrar enteros escalados
            seq_printf(m, "      \"cpu_usage\": %lu.%03lu%%\n", cpu_usage_scaled / 1000, cpu_usage_scaled % 1000); // Mostrar enteros escalados
            seq_printf(m, "    },\n");
        }
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