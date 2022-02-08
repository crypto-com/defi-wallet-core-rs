#define _CRT_SECURE_NO_WARNINGS
#include <iostream>
#include <cstdlib>
#include "bindings.h"
int main()
{
    int8_t error[100];
    const char* mnemonics = std::getenv("MYMNEMONICS");
    Wallet* wallet = restore_wallet(mnemonics, "", error, 100);
    if (NULL == wallet) {
        std::cout << error << std::endl;
        std::cout << "declare env variable\nexport MYMNEMONICS=yourmnemonics" << std::endl;
        exit(-1);
    }
    int8_t buffer[100];
    bool get_address_result=get_address(wallet, buffer, 100, error, 100);
    if (!get_address_result) {
        std::cout << error << std::endl;
        exit(-1);
    }
    std::cout <<"wallet address = " << buffer << std::endl;
    destroy_wallet(wallet);
}
