#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/mm.h>
#include <linux/sched.h>
#include <linux/sched/signal.h>  
#include <linux/timer.h> 
#include <linux/jiffies.h> 

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Dani");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y procesos padre e hijos");
MODULE_VERSION("1.0");

#define PROC_NAME "inform"

static int inform_show(struct seq_file *m, void *v) {
    struct sysinfo si;
    si_meminfo(&si);
    seq_printf(m, "Realizado por Dani\nDatos de memoria:\n");
    seq_printf(m, "Total RAM: %lu KB\n", si.totalram * (si.mem_unit / 1024));
    seq_printf(m, "Libre RAM: %lu KB\n", si.freeram * (si.mem_unit / 1024));

    struct task_struct *task;
    seq_printf(m, "\nProcesos Padre e Hijos:\n");
    for_each_process(task) {
        if (!list_empty(&task->children)) {  // Solo considerar procesos con hijos
            seq_printf(m, "Padre: %s [PID: %d]\n", task->comm, task->pid);
            struct task_struct *child;
            list_for_each_entry(child, &task->children, sibling) {
                seq_printf(m, "    Hijo: %s [PID: %d]\n", child->comm, child->pid);
            }
            seq_printf(m, "\n");
        }
    }

    return 0;
}

static int inform_open(struct inode *inode, struct file *file) {
    return single_open(file, inform_show, NULL);
}

static const struct proc_ops inform_ops = {
    .proc_open = inform_open,
    .proc_read = seq_read,
};

static int __init inform_init(void) {
    proc_create(PROC_NAME, 0, NULL, &inform_ops);
    printk(KERN_INFO "inform module loaded\n");
    return 0;
}

static void __exit inform_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "inform module unloaded\n");
}

module_init(inform_init);
module_exit(inform_exit);
