#include "CPakInterface.h"
#include "PakInterface.h"

extern "C" {
/// Utility
bool has_filetime(PakRecord *record) {
  return (record->mFileTime.dwLowDateTime != 0) &&
         (record->mFileTime.dwHighDateTime !=
          0); // Reading pak files causes crashes because why not so lets be
              // conservative and not load files without datetimes (signal of
              // loaded pak file)
}

struct pak_interface_base_t {
  PakInterfaceBase *base;
};

struct pak_interface_t {
  PakInterface *interface;
};

struct pak_file_t {
  PFILE *file;
};

pak_interface_base_t *get_pak_ptr() {
  pak_interface_base_t *base = new pak_interface_base_t;
  base->base = GetPakPtr();
  return base;
}

/////////////////////////////////
/// pak_interface_t functions ///
/////////////////////////////////

pak_interface_t *pak_interface_create() {
  pak_interface_t *interface = new pak_interface_t;
  interface->interface = new PakInterface();
  return interface;
}

bool pak_interface_add_pak_file(pak_interface_t *interface, const char *path) {
  return interface->interface->AddPakFile(path);
}

pak_file_t *pak_interface_fopen(pak_interface_t *interface, const char *name,
                                const char *access) {
  PFILE *file = interface->interface->FOpen(name, access);
  if (file == NULL || !has_filetime(file->mRecord))
    return NULL;
  pak_file_t *wrapper = new pak_file_t;
  wrapper->file = file;

  return wrapper;
}

int pak_interface_fclose(pak_interface_t *interface, pak_file_t *file) {
  int ret = interface->interface->FClose(file->file);
  delete file;
  return ret;
}

int pak_interface_fseek(pak_interface_t *interface, pak_file_t *file,
                        long offset, int origin) {
  return interface->interface->FSeek(file->file, offset, origin);
}

int pak_interface_ftell(pak_interface_t *interface, pak_file_t *file) {
  return interface->interface->FTell(file->file);
}

size_t pak_interface_fread(pak_interface_t *interface, void *ptr, int el_size,
                           int count, pak_file_t *file) {
  return interface->interface->FRead(ptr, el_size, count, file->file);
}

int pak_interface_feof(pak_interface_t *interface, pak_file_t *file) {
  return interface->interface->FEof(file->file);
}

HANDLE pak_interface_find_first_file(pak_interface_t *interface,
                                     LPCSTR file_name, LPWIN32_FIND_DATA data) {
  return interface->interface->FindFirstFile(file_name, data);
}

bool pak_interface_find_next_file(pak_interface_t *interface, HANDLE handle,
                                  LPWIN32_FIND_DATA data) {
  return interface->interface->FindNextFile(handle, data);
}

bool pak_interface_find_close(pak_interface_t *interface, HANDLE handle) {
  return interface->interface->FindClose(handle);
}

void pak_interface_destroy(pak_interface_t *p) {
  delete p->interface;
  delete p;
}

bool pak_interface_list_all_file_paths(pak_interface_t *interface,
                                       const char **data, size_t *buffer_size) {
  size_t data_size = 0;
  for (PakRecordMap::iterator it = interface->interface->mPakRecordMap.begin();
       it != interface->interface->mPakRecordMap.end(); it++)
    if (has_filetime(&it->second))
      data_size++;

  if (data_size > *buffer_size) {
    *buffer_size = data_size;
    return false;
  }

  if (data == NULL)
    return false;

  PakRecordMap::iterator it = interface->interface->mPakRecordMap.begin();
  for (size_t i = 0; it != interface->interface->mPakRecordMap.end(); it++) {
    if (has_filetime(&it->second)) {
      data[i] = it->first.c_str();
      i++;
    }
  }

  return true;
}
}