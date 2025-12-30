#include <iostream>
#include <vector>

int main() {
    std::vector<int> numbers = {1, 2, 3};
    
    // 最初の要素への参照を取得
    int& first = numbers[0];
    
    std::cout << "追加前の最初の要素: " << first << std::endl;

    // 大量に要素を追加して、メモリの再確保を強制的に発生させる
    for (int i = 0; i < 1000; ++i) {
        numbers.push_back(i);
    }

    // 再確保が起きると、'first' が指していた古いメモリ領域は解放されている
    // しかし、C++ はこれへのアクセスを禁止しない（ダングリング参照）
    std::cout << "追加後の最初の要素（危険）: " << first << std::endl; 

    return 0;
}