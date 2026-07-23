// #[derive(Wrapper)]
// struct UnitStruct;
//
// #[derive(Wrapper)]
// struct Struct
// {
//   #[wrapper(inner)]
//   pub inner: u32,
//   #[wrapper(_inner)]
//   inner2:    u64,
//   #[wrapper(a, inner)]
//   inner3:    u128,
// }
//
// #[derive(Wrapper)]
// struct Struct2
// {
//   #[wrapper]
//   inner:      u32,
//   #[wrapper(_inner)]
//   inner2:     u64,
//   #[wrapper(inner)]
//   pub inner3: u128,
//   #[wrapper(inner)]
//   pub inner4: u128,
// }
//
// #[derive(Wrapper)]
// struct Struct3(#[wrapper] u32, #[wrapper(_inner)] u64, #[wrapper(inner)]
// u128);
//
// #[derive(Wrapper)]
// struct StructT<'a, T, const M: usize, const N: usize>(#[wrapper(inner)] &'a
// T);
