

library(helloextendr)

#some error messages and a valid return value
usize_add_42(-1)
usize_add_42(complex(1))
usize_add_42(5)


# many calls to rust and combined error handling on R side
usize_add_42_convoluted(5)
usize_add_42_convoluted(-1)


# show case SuperClassMe wrapper
usize_add_42_superclassy()

                        