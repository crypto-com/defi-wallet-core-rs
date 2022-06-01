#include "defi-wallet-core-cpp/include/nft.h"

namespace org {
namespace defi_wallet_core {
Pagination::Pagination()
    : enable{false}, offset{0}, limit{100}, count_total{false}, reverse{false} {
}

bool Pagination::get_enable() const { return enable; }
Vec<uint8_t> Pagination::get_key() const { return key; }
uint64_t Pagination::get_offset() const { return offset; }
uint64_t Pagination::get_limit() const { return limit; }
bool Pagination::get_count_total() const { return count_total; }
bool Pagination::get_reverse() const { return reverse; }
} // namespace defi_wallet_core
} // namespace org
