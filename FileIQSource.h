#pragma once

#include"IQSource.h"
#include<fstream>

class FileIQSource : public IQSource {
public:
    FileIQSource(const char * path);
    size_t read(IQ* buff, size_t len) override;

private:
    std::ifstream file;
};

