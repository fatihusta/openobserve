// Copyright 2022 Zinc Labs Inc. and Contributors
//
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

use datafusion::error::Result;

use crate::service::promql::value::{RangeValue, Value};

pub(crate) fn irate(data: &Value) -> Result<Value> {
    super::eval_idelta(data, "irate", exec)
}

fn exec(data: &RangeValue) -> Option<f64> {
    if data.samples.len() < 2 {
        return None;
    }
    let (last, data) = data.samples.split_last().unwrap();
    let previous = data.last().unwrap();
    let dt_seconds = ((last.timestamp - previous.timestamp) / 1_000_000) as f64;
    if dt_seconds == 0.0 {
        return Some(0.0);
    }
    Some((last.value - previous.value) / dt_seconds)
}