#include <iostream>

void win(int counter) {
    if (counter >= 10000000) {
        std::cout << "win!!!" << std::endl;
    }
}

int main()
{
    std::cout << "Simple Counter!!!!\n";
    int counter = 0;
    std::cout << "counter address: 0x" << &counter << std::endl;
    std::cout << "win address: 0x" << &win << std::endl;

    while (1) {
        win(counter);
        getchar();
        std::cout << "counter:" << counter << std::endl;
        counter++;
    }
}