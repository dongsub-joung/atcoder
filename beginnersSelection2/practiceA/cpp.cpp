#include <cstdio>
#include <iostream>
#include <string>

int main() {
  // declear
  int a,b,c;
  std::string name;

  // allocate
  std::cin >> a;

  std::scanf("%d", &b);
  std::scanf("%d", &c);

  std::cin >> name;

  // @todo
  // https://releases.llvm.org/14.0.0/projects/libcxx/docs/ReleaseNotes.html#id3
  // -DLIBCXX_ENABLE_INCOMPLETE_FEATURES=ON when configuring their build.
  // std::string result= std::format("{} {}", a+b+c, name);

  std::cout << a+b+c
            << " "
            << name 
            << std::endl;
  return 0;
}
