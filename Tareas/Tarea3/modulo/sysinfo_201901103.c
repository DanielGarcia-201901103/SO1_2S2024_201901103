#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/sched.h>
#include <linux/sched/signal.h>
#include <linux/mm.h>
#include <linux/slab.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Dani");
MODULE_DESCRIPTION("Módulo del kernel para capturar métricas de memoria y contenedores en JSON");
MODULE_VERSION("1.0");

#define PROC_FILENAME "sysinfo_201901103"

static int sysinfo_show(struct seq_file *m, void *v) {
    struct task_struct *task;
    unsigned long ram_usage;
    unsigned long total_jiffies = jiffies;  // jiffies actual
    unsigned long task_cpu_time;
    unsigned long long cpu_usage;

    seq_printf(m, "{\n");
    seq_printf(m, "  \"containers\": [\n");

    // Recorrer todos los procesos en el sistema
    for_each_process(task) {
        if (strstr(task->comm, "container")) {  // Filtro básico para procesos de contenedores
            struct mm_struct *mm = task->mm;
            unsigned long vm_rss;

            // Calcular el uso de CPU para este proceso
            task_cpu_time = (task->utime + task->stime);
            cpu_usage = (1000 * task_cpu_time) / total_jiffies;

            // Calcular el uso de RAM para este proceso
            if (mm) {
                vm_rss = get_mm_rss(mm) << (PAGE_SHIFT - 10); // KB
                ram_usage = vm_rss; // en KB
            } else {
                ram_usage = 0;
            }

            seq_printf(m, "    {\n");
            seq_printf(m, "      \"id\": \"%s\",\n", task->comm);
            seq_printf(m, "      \"cpu_usage\": %llu.%llu,\n", cpu_usage / 10, cpu_usage % 10);
            seq_printf(m, "      \"ram_usage\": %lu\n", ram_usage);
            seq_printf(m, "    },\n");
        }
    }

    seq_printf(m, "  ]\n");
    seq_printf(m, "}\n");

    return 0;
}

static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

static const struct proc_ops sysinfo_fops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release,
};

static int __init sysinfo_init(void) {
    proc_create(PROC_FILENAME, 0, NULL, &sysinfo_fops);
    printk(KERN_INFO "sysinfo module loaded\n");
    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_FILENAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);
