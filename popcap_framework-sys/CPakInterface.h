#ifndef _CPAKINERFACE_H_
#define _CPAKINERFACE_H_
#define WIN32_LEAN_AND_MEAN
#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct pak_interface_base_t pak_interface_base_t;
typedef struct pak_interface_t pak_interface_t;
typedef struct pak_file_t pak_file_t;

/// Returns Global pak_interface_base_t ptr
pak_interface_base_t *get_pak_ptr();

/////////////////////////////////
/// pak_interface_t functions ///
/////////////////////////////////

/// Create a local pak_interface_t
pak_interface_t *pak_interface_create();

/// Load a pak file into the interface. Returns false on failure.
bool pak_interface_add_pak_file(pak_interface_t *interface, const char *path);

/// Open a file. Returns NULL on failure.
pak_file_t *pak_interface_fopen(pak_interface_t *interface, const char *name,
                                const char *access);

/// Close a file. Returns 0 if successful.
int pak_interface_fclose(pak_interface_t *interface, pak_file_t *file);

/// Run a classic fseek on a pak file
int pak_interface_fseek(pak_interface_t *interface, pak_file_t *file,
                        long offset, int origin);

/// Run a classic ftell on a pak file
int pak_interface_ftell(pak_interface_t *interface, pak_file_t *file);

/// Run a classic fread on a pak file
size_t pak_interface_fread(pak_interface_t *interface, void *ptr, int el_size,
                           int count, pak_file_t *file);

/// Run a classic feof on a pak file
int pak_interface_feof(pak_interface_t *interface, pak_file_t *file);

/// Find a file by a pattern
HANDLE pak_interface_find_first_file(pak_interface_t *interface,
                                     LPCSTR file_name, LPWIN32_FIND_DATA data);

/// Find the next file from a search
bool pak_interface_find_next_file(pak_interface_t *interface, HANDLE handle,
                                  LPWIN32_FIND_DATA data);

/// Close a search handle
bool pak_interface_find_close(pak_interface_t *interface, HANDLE handle);

/// Destroy a local pak_interface_t
void pak_interface_destroy(pak_interface_t *p);

/// Get a list of all loose files. Returns false if data is null or size is too
/// small. Size is populated with the correct size for the buffer.
bool pak_interface_list_all_file_paths(pak_interface_t *interface,
                                       const char **data, size_t *size);

#ifdef __cplusplus
}
#endif

#endif