#include "dog.hpp"
#include <cstdio>

Dog::Dog(const char *name) : _name(name), _status(STATUS_STOP) {
  std::printf("%s: bow-wow.\n", _name);
}

Dog::~Dog() { std::printf("%s: bow-wow.\n", _name); }

void Dog::walk() {
  std::printf("%s: bow-wow!\n", _name);
  if (_status != STATUS_WALKING)
    _status = STATUS_WALKING;
}

void Dog::stop() {
  std::printf("%s: bow-wow.\n", _name);
  if (_status != STATUS_STOP)
    _status = STATUS_STOP;
}
