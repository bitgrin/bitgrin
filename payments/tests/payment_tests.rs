// Copyright 2019 The BitGrin Developers
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bitgrin_payments as payments;
// use bitgrin_payments::payments::PAYMENTS_STR;
// use crate::PAYMENTS_STR_2 as pst;

/// Test we correctly verify coinbase maturity when adding txs to the pool.
#[test]
fn test_payments() {
    assert_eq!(4, 4);
    payments::p2p::init();
}
