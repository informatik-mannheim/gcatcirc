# Copyright 2021 by the authors.
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
# 
#     http://www.apache.org/licenses/LICENSE-2.0
#  
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License. 

# Test file
library(testthat)

context("c3 codes") # Infos

test_that("X0 Code", {
  X0 = all_c3_codes[[23]]
  expect_equal(get.id(X0), "X23")
})

test_that("X0 Code alone", {
  X0 = c3_code(23) # does not work
  expect_equal(get.id(X0), "X23")
  #expect_equal("X23", "X23") # TODO
})