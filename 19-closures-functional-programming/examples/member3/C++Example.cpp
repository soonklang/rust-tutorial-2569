#include <iostream>
using namespace std;

int main() {
    int n = 10;

    auto addN = [n](int x) {
        return x + n;
    };

    cout << addN(5) << endl; // 15
}
