fn same ( i : u32 ) -> u32 {
if i == 0 {
0
} else {
1 + same ( i - 1)
}
}
fn main () {
println !("{}" , same (4) ) ;
//test_same () ;
}

fn test_same () {
assert_eq! ( same (0) , 0 ) ;
assert_eq! ( same (1) , 1 ) ;
assert_eq! ( same (2) , 2 ) ;
assert_eq! ( same (3) , 3 ) ;
assert_eq! ( same (4) , 4 ) ;
}