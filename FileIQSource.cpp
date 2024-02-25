//
// Created by 16182 on 2/19/2024.
//

#include "FileIQSource.h"

FileIQSource::FileIQSource(const char *path) {
    file = std::ifstream(path, std::ios::binary);
}

size_t FileIQSource::read(IQ *buff, size_t len) {
    file.read((char*)buff, len * sizeof(IQ));
    size_t ct = file.gcount();
    return ct / sizeof(IQ);
}
