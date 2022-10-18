Arithmetic operators

std::ops::Add x + y
std::ops::Sub x - y
std::ops::Mul x * y
std::ops::Div x / y
std::ops::Rem x % y

Bitwise operators

std::ops::BitAnd x & y
std::ops::BitOr x | y
std::ops::BitXor x ^ y
std::ops::Shl x << y
std::ops::Shr x >> y

Compound assignment arithmetic operators

std::ops::AddAssign x += y
std::ops::SubAssign x -= y
std::ops::MulAssign x *= y
std::ops::DivAssign x /= y
std::ops::RemAssign x %= y

Compound assignment bitwise operators

std::ops::BitAndAssign x &= y
std::ops::BitOrAssign x |= y
std::ops::BitXorAssign x ^= y
std::ops::ShlAssign x <<= y
std::ops::ShrAssign x >>= y
Comparison

std::cmp::PartialEq x == y, x != y
std::cmp::PartialOrd x < y,  x <= y,  x > y,  x >= y
Indexing std::ops::Index x[y],  &x[y]
std::ops::IndexMut x[y] = z,  &mut x[y]
