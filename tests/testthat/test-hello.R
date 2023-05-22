test_that("no unit tests see test_does_it_leak", {
  expect_equal(2+2, 4)
  print(getwd())
  source("../test_does_it_leak.R")
  
  expect_equal(2+2, 4)
})
