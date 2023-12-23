#include <filesystem>
#include <iostream>
int main() {
  std::filesystem::recursive_directory_iterator iter("src");
  for (auto f : iter) {
    std::cout << f.path() << ' ' << f.is_directory();
    if (f.is_regular_file())
      std::cout << ' ' << f.file_size();
    std::cout << std::endl;
  }
}