#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Wallet;

extern "C" {

Wallet *restore_wallet(const char *mnemonic,
                       const char *passphrase,
                       int8_t *error,
                       int32_t error_length);

bool get_address(Wallet *wallet,
                 int8_t *out_address,
                 int32_t out_address_length,
                 int8_t *error,
                 int32_t error_length);

void destroy_wallet(Wallet *wallet);

} // extern "C"
