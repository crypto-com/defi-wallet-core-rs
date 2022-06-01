#pragma once
#include "rust/cxx.h"
#include <memory>

namespace org {
namespace defi_wallet_core {

using namespace rust;

/// struct for efficient pagination
struct Pagination {
  /// Set true to enable the pagination
  ///
  /// A workaround filed for enabling pagination. It could be changed to
  /// std::optional<Pagination> if https://github.com/dtolnay/cxx/issues/87 fixed
  bool enable;
  /// key is a value returned in PageResponse.next_key to begin
  /// querying the next page most efficiently. Only one of offset or key
  /// should be set.
  Vec<uint8_t> key;
  /// offset is a numeric offset that can be used when key is
  /// unavailable. It is less efficient than using key. Only
  /// one of offset or key should be set.
  uint64_t offset;
  /// limit is the total number of results to be returned in the
  /// result page. If left empty it will default to a value to be
  /// set by each app.
  uint64_t limit;
  /// count_total is set to true  to indicate that the result set
  /// should include a count of the total number of items available
  /// for pagination in UIs. count_total is only respected when
  /// offset is used. It is ignored when key is set.
  bool count_total;
  /// reverse is set to true if results are to be
  /// returned in the descending order.
  ///
  /// Since: cosmos-sdk 0.43
  bool reverse;

  Pagination();

  bool get_enable() const;
  Vec<uint8_t> get_key() const;
  uint64_t get_offset() const;
  uint64_t get_limit() const;
  bool get_count_total() const;
  bool get_reverse() const;
};

} // namespace defi_wallet_core
} // namespace org
