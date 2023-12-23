#include <filesystem>
#include <iostream>
int main() {
  std::filesystem::recursive_directory_iterator iter("src");
  for (auto f : iter) {
    std::cout << f.path() << std::endl;
  }
}