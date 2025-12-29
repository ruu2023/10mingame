#include <iostream>
#include <vector>

int main() {
    // std::vector は Laravel の Collection のようなものですが、
    // メモリの確保と解放を内部で厳格に行っています。
    std::vector<std::string> messages = {"Hello", "C++", "Speed"};
    
    for(const auto& msg : messages) {
        std::cout << msg << "!" << std::endl;
    }
    return 0;
}