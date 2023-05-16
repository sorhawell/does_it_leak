#' @export
usize_add_42 = function(x) {
  helloextendr:::usize_add_42_context_blame(x) |> unwrap()
}

#' @export
usize_add_42_convoluted = function(x) {
  f = helloextendr:::usize_add_42_blame_macro
  f(x) |> and_then(f) |> and_then(f) |> unwrap( context = "in usize_add_42_convoluted():")
}


#' @export
usize_add_42_superclassy = function() {
  helloextendr:::usize_add_42_superclass()
}


#' @export
print.MyStruct = function(x, ...) {
  x$print()
  str(x)
}