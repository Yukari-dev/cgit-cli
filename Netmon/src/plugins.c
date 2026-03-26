#include "plugins.h"
#include <dirent.h>
#define MAX_PLUGINS 32
static PyObject *plugins[MAX_PLUGINS];
static int plugin_count = 0;

void plugins_init(const char *plugins_dir){
    Py_Initialize();
    PyObject *sys_path = PySys_GetObject("path");

    PyObject *py_dir = PyUnicode_FromString(plugins_dir);
    PyList_Append(sys_path, py_dir);
    Py_DECREF(py_dir);

    DIR *dir = opendir(plugins_dir);
    if(dir == NULL) return;
    struct dirent *entry;
    while((entry = readdir(dir)) != NULL){
        char *name = entry->d_name;
        int len = strlen(name); 
        if(len > 3 && strcmp(name + len - 3, ".py") == 0){
            char module_name[256];
            strncpy(module_name, name, len - 3);
            module_name[len - 3] = '\0';
            PyObject *module = PyImport_ImportModule(module_name);
            if(module != NULL){
                plugins[plugin_count++] = module;
            }
        }
    }
    
    closedir(dir);
}

void plugins_on_packet(const char *src_ip, const char *dest_ip, const char *protocol, int size){
    for(int i = 0; i < plugin_count; i++){
        PyObject *func = PyObject_GetAttrString(plugins[i], "on_packet");
        if(func && PyCallable_Check(func)){
            PyObject *result = PyObject_CallFunction(func, "sssi", src_ip, dest_ip, protocol, size);
            Py_XDECREF(result);
        }
        Py_XDECREF(func);
    }
}

void plugins_cleanup(void){
    for(int i = 0; i < plugin_count; i++){
        Py_DECREF(plugins[i]);
    }
    Py_Finalize();
}

