(function() {var implementors = {
"anyhow":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a>&gt; for <a class=\"struct\" href=\"anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt; for <a class=\"struct\" href=\"anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>"]],
"bytes":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"bytes/struct.Bytes.html\" title=\"struct bytes::Bytes\">Bytes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"bytes/struct.BytesMut.html\" title=\"struct bytes::BytesMut\">BytesMut</a>"]],
"deranged":[["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u64.html\">u64</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u64.html\">u64</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedU64.html\" title=\"struct deranged::RangedU64\">RangedU64</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u16.html\">u16</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u16.html\">u16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u16.html\">u16</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedU16.html\" title=\"struct deranged::RangedU16\">RangedU16</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i64.html\">i64</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i64.html\">i64</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i64.html\">i64</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedI64.html\" title=\"struct deranged::RangedI64\">RangedI64</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u128.html\">u128</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u128.html\">u128</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedU128.html\" title=\"struct deranged::RangedU128\">RangedU128</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedUsize.html\" title=\"struct deranged::RangedUsize\">RangedUsize</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.isize.html\">isize</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.isize.html\">isize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.isize.html\">isize</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedIsize.html\" title=\"struct deranged::RangedIsize\">RangedIsize</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i128.html\">i128</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i128.html\">i128</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i128.html\">i128</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedI128.html\" title=\"struct deranged::RangedI128\">RangedI128</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedU8.html\" title=\"struct deranged::RangedU8\">RangedU8</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i32.html\">i32</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i32.html\">i32</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i32.html\">i32</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedI32.html\" title=\"struct deranged::RangedI32\">RangedI32</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i8.html\">i8</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i8.html\">i8</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i8.html\">i8</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedI8.html\" title=\"struct deranged::RangedI8\">RangedI8</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedU32.html\" title=\"struct deranged::RangedU32\">RangedU32</a>&lt;MIN, MAX&gt;"],["impl&lt;const MIN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a>, const MAX: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.i16.html\">i16</a>&gt; for <a class=\"struct\" href=\"deranged/struct.RangedI16.html\" title=\"struct deranged::RangedI16\">RangedI16</a>&lt;MIN, MAX&gt;"]],
"gotham":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"gotham/helpers/http/struct.PercentDecoded.html\" title=\"struct gotham::helpers::http::PercentDecoded\">PercentDecoded</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"gotham/helpers/http/struct.FormUrlDecoded.html\" title=\"struct gotham::helpers::http::FormUrlDecoded\">FormUrlDecoded</a>"]],
"http":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/method/struct.Method.html\" title=\"struct http::method::Method\">Method</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"http/header/struct.HeaderValue.html\" title=\"struct http::header::HeaderValue\">HeaderValue</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/header/struct.HeaderName.html\" title=\"struct http::header::HeaderName\">HeaderName</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"http/header/struct.HeaderName.html\" title=\"struct http::header::HeaderName\">HeaderName</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/uri/struct.Scheme.html\" title=\"struct http::uri::Scheme\">Scheme</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/uri/struct.Port.html\" title=\"struct http::uri::Port\">Port</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt;,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/uri/struct.Authority.html\" title=\"struct http::uri::Authority\">Authority</a>"]],
"hyper":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"hyper/ext/struct.ReasonPhrase.html\" title=\"struct hyper::ext::ReasonPhrase\">ReasonPhrase</a>"]],
"mime":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"mime/struct.Mime.html\" title=\"struct mime::Mime\">Mime</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"mime/struct.Name.html\" title=\"struct mime::Name\">Name</a>&lt;'a&gt;"]],
"powerfmt":[["impl&lt;const SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"powerfmt/buf/struct.WriteBuffer.html\" title=\"struct powerfmt::buf::WriteBuffer\">WriteBuffer</a>&lt;SIZE&gt;"],["impl&lt;const SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"powerfmt/buf/struct.WriteBuffer.html\" title=\"struct powerfmt::buf::WriteBuffer\">WriteBuffer</a>&lt;SIZE&gt;"]],
"regex_syntax":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"regex_syntax/hir/literal/struct.Literal.html\" title=\"struct regex_syntax::hir::literal::Literal\">Literal</a>"]],
"time":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/std/time/struct.Instant.html\" title=\"struct std::time::Instant\">Instant</a>&gt; for <a class=\"struct\" href=\"time/struct.Instant.html\" title=\"struct time::Instant\">Instant</a>"]],
"tinyvec":[["impl&lt;A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[&lt;A as <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt;::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>]&gt; for <a class=\"enum\" href=\"tinyvec/enum.TinyVec.html\" title=\"enum tinyvec::TinyVec\">TinyVec</a>&lt;A&gt;"],["impl&lt;'s, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.slice.html\">[T]</a>&gt; for <a class=\"struct\" href=\"tinyvec/struct.SliceVec.html\" title=\"struct tinyvec::SliceVec\">SliceVec</a>&lt;'s, T&gt;"],["impl&lt;A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[&lt;A as <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt;::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>]&gt; for <a class=\"struct\" href=\"tinyvec/struct.ArrayVec.html\" title=\"struct tinyvec::ArrayVec\">ArrayVec</a>&lt;A&gt;"]],
"tokio":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.UnixStream.html\" title=\"struct tokio::net::UnixStream\">UnixStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/unix/struct.OwnedReadHalf.html\" title=\"struct tokio::net::unix::OwnedReadHalf\">OwnedReadHalf</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.UnixStream.html\" title=\"struct tokio::net::UnixStream\">UnixStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/unix/struct.WriteHalf.html\" title=\"struct tokio::net::unix::WriteHalf\">WriteHalf</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.UnixStream.html\" title=\"struct tokio::net::UnixStream\">UnixStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/unix/struct.OwnedWriteHalf.html\" title=\"struct tokio::net::unix::OwnedWriteHalf\">OwnedWriteHalf</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.TcpStream.html\" title=\"struct tokio::net::TcpStream\">TcpStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/tcp/struct.OwnedWriteHalf.html\" title=\"struct tokio::net::tcp::OwnedWriteHalf\">OwnedWriteHalf</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.TcpStream.html\" title=\"struct tokio::net::TcpStream\">TcpStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/tcp/struct.OwnedReadHalf.html\" title=\"struct tokio::net::tcp::OwnedReadHalf\">OwnedReadHalf</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.UnixStream.html\" title=\"struct tokio::net::UnixStream\">UnixStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/unix/struct.ReadHalf.html\" title=\"struct tokio::net::unix::ReadHalf\">ReadHalf</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.TcpStream.html\" title=\"struct tokio::net::TcpStream\">TcpStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/tcp/struct.ReadHalf.html\" title=\"struct tokio::net::tcp::ReadHalf\">ReadHalf</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"tokio/net/struct.TcpStream.html\" title=\"struct tokio::net::TcpStream\">TcpStream</a>&gt; for <a class=\"struct\" href=\"tokio/net/tcp/struct.WriteHalf.html\" title=\"struct tokio::net::tcp::WriteHalf\">WriteHalf</a>&lt;'_&gt;"]],
"tracing_core":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"tracing_core/struct.Field.html\" title=\"struct tracing_core::Field\">Field</a>"]],
"unicase":[["impl&lt;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.str.html\">str</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"unicase/struct.Ascii.html\" title=\"struct unicase::Ascii\">Ascii</a>&lt;S&gt;"],["impl&lt;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.str.html\">str</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/core/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"unicase/struct.UniCase.html\" title=\"struct unicase::UniCase\">UniCase</a>&lt;S&gt;"]],
"url":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"url/struct.Url.html\" title=\"struct url::Url\">Url</a>"]],
"uuid":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"struct\" href=\"uuid/fmt/struct.Urn.html\" title=\"struct uuid::fmt::Urn\">Urn</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"struct\" href=\"uuid/fmt/struct.Hyphenated.html\" title=\"struct uuid::fmt::Hyphenated\">Hyphenated</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"struct\" href=\"uuid/fmt/struct.Simple.html\" title=\"struct uuid::fmt::Simple\">Simple</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"struct\" href=\"uuid/fmt/struct.Braced.html\" title=\"struct uuid::fmt::Braced\">Braced</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]&gt; for <a class=\"struct\" href=\"uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()