" M0RX Language Support for Vim
au BufRead,BufNewFile *.mrx set filetype=m0rx
syntax keyword m0rxKeyword if else elif match when loop while each break skip give halt
syntax keyword m0rxKeyword fn method class trait impl let fix bind mod use
syntax keyword m0rxKeyword async await spawn try catch toss panic
syntax keyword m0rxKeyword model infer teach tensor flow embed predict
syntax keyword m0rxType tiny short ant long vast half dbl precise str txt bool nil list map set
syntax keyword m0rxConst true false null void
syntax region m0rxString start=/"/ end=/"/
syntax match m0rxComment "//.*$"
syntax match m0rxNumber "\b[0-9]\+\(\.[0-9]\+\)\?\b"
hi def link m0rxKeyword Keyword
hi def link m0rxType Type
hi def link m0rxConst Constant
hi def link m0rxString String
hi def link m0rxComment Comment
hi def link m0rxNumber Number
