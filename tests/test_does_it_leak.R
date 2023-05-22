library(helloextendr)
library(lobstr)

e_pkg = environment(helloextendr:::implicit_string)

fnames = c(
  
  "implicit_double",# single burn-in function
  
  "implicit_double",
  "implicit_double",
  "implicit_doubles",
  "implicit_string",
  "implicit_strings",
  
  "arg2_try_implicit_double",
  "arg2_try_implicit_doubles",
  "arg2_try_implicit_string",
  "arg2_try_implicit_strings",
  
  "try_implicit_double",
  "try_implicit_doubles",
  "try_implicit_string",
  "try_implicit_strings",

  "unwrap_double",
  "unwrap_doubles",
  "unwrap_string",
  "unwrap_strings",
  "error_double",
  "error_doubles",
  "error_string",
  "error_strings"
)
rust_f_list = mget(fnames,envir = e_pkg)
names(rust_f_list)[1] = "burn_in_test" #rename burn-in

value_f_list = list(
  big_string = \() paste(sample(letters,1E3, replace = TRUE),collapse = ""),
  big_chrvec = \() replicate(1E1,paste(sample(letters,1E2, replace = TRUE),collapse = "")),
  big_intvec = \() (1:(1E3)) - 1L,
  big_altvec = \() 1:(1E3),
  big_dblvec = \() (1:(1E3)) - 1.1,
  string = \() "hey mom",
  int = 42L,
  dbl = 42.42
)

##global memory
glb_i = 0
glb_mem_before      = NA_real_
glb_mem_before_10   = NA_real_
glb_mem_after       = NA_real_
glb_mem_after_10    = NA_real_
glb_mem_after_gc    = NA_real_
glb_mem_after_gc_10 = NA_real_
glb_is_error = NA


score_leak <- function(f_rust, f_value){
  
  cat(glb_i<<-glb_i+1,", ",sep="")
  glb_mem_before <<- lobstr::mem_used()
  
  print(f_rust)
  print(f_value)
  
  out = (\() tryCatch({
      if(length(formals(f_rust))>1 ) {
        f_rust(rnorm(1E3),f_value())
      } else {
        f_rust(f_value())
      }
    }, error = \(err) "ERROR"))()
  glb_mem_after <<- lobstr::mem_used()
  glb_is_error <<- isTRUE(out == "ERROR")
  rm(out)
  gc(verbose = FALSE)
  glb_mem_after_gc = lobstr::mem_used()
  
  #run 10 times
  glb_mem_before_10 <<- lobstr::mem_used()
  for (i in 1:10) {
    cat(".")
    out = (\() tryCatch({
      if(length(formals(f_rust))>1 ) {
        f_rust(rnorm(1E4),f_value())
      } else {
        f_rust(f_value())
      }
    }, error = \(err) "ERROR"))()
  }
  glb_mem_after_10 <<- lobstr::mem_used()
  rm(out,i)
  gc(verbose = FALSE)
  glb_mem_after_gc_10 <<- lobstr::mem_used()
  
  l = list(
    total_mem_before = glb_mem_before,
    is_error = glb_is_error,
    leak_size_1  = glb_mem_after_gc -glb_mem_before,
    leak_size_10 = (glb_mem_after_gc_10 - glb_mem_before_10) / 10
  )
  
  l
}

zip = \(x,y) lapply(seq_along(x), \(i) list(x[[i]],y[[i]]))
zip_names = \(x) zip(x, names(x))
flatten = \(x) do.call(c,x)
unlist_elements = \(x) {x[]=lapply(x,unlist);x}

#capture error messages, because large inputs spam the log
try(unlink("errout.txt"))
con = textConnection("errout.txt")
sink(file =con, append = FALSE, type = c("message"),split = FALSE)


#perform benchmark
mem_bench = 
    lapply(zip_names(rust_f_list), \(f_rust)  {
      lapply(zip_names(value_f_list), \(f_val) {
        c(
          list(f_name = f_rust[[2]],val_name = f_val[[2]]),
          score_leak(f_rust[[1]], f_val[[1]])
        )
      })
    })

#gather results
mem_bench_table = mem_bench |>
  flatten() |>
  (\(x) do.call(rbind, args = x))() |>
  data.frame() |>
  unlist_elements()


#stop divert err msg
sink(NULL, type = c("message"))

#print result for log
print(mem_bench_table, max=9999)

#write to file
sink("../leak_result.txt")
print(mem_bench_table, max=9999)
sink(NULL)

