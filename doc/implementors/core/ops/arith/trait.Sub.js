(function() {var implementors = {};
implementors["time"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Date.html\" title=\"struct time::Date\">Date</a>","synthetic":false,"types":["time::date::Date"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Date.html\" title=\"struct time::Date\">Date</a>","synthetic":false,"types":["time::date::Date"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Date.html\" title=\"struct time::Date\">Date</a>&gt; for <a class=\"struct\" href=\"time/struct.Date.html\" title=\"struct time::Date\">Date</a>","synthetic":false,"types":["time::date::Date"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>","synthetic":false,"types":["time::duration::Duration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>","synthetic":false,"types":["time::duration::Duration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">StdDuration</a>","synthetic":false,"types":["core::time::Duration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Instant.html\" title=\"struct time::Instant\">Instant</a>&gt; for <a class=\"struct\" href=\"time/struct.Instant.html\" title=\"struct time::Instant\">Instant</a>","synthetic":false,"types":["time::instant::Instant"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/std/time/struct.Instant.html\" title=\"struct std::time::Instant\">Instant</a>&gt; for <a class=\"struct\" href=\"time/struct.Instant.html\" title=\"struct time::Instant\">Instant</a>","synthetic":false,"types":["time::instant::Instant"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Instant.html\" title=\"struct time::Instant\">Instant</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/std/time/struct.Instant.html\" title=\"struct std::time::Instant\">StdInstant</a>","synthetic":false,"types":["std::time::Instant"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Instant.html\" title=\"struct time::Instant\">Instant</a>","synthetic":false,"types":["time::instant::Instant"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/std/time/struct.Instant.html\" title=\"struct std::time::Instant\">StdInstant</a>","synthetic":false,"types":["std::time::Instant"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Instant.html\" title=\"struct time::Instant\">Instant</a>","synthetic":false,"types":["time::instant::Instant"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;T&gt; for <a class=\"struct\" href=\"time/struct.OffsetDateTime.html\" title=\"struct time::OffsetDateTime\">OffsetDateTime</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"time/struct.PrimitiveDateTime.html\" title=\"struct time::PrimitiveDateTime\">PrimitiveDateTime</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;T, Output = <a class=\"struct\" href=\"time/struct.PrimitiveDateTime.html\" title=\"struct time::PrimitiveDateTime\">PrimitiveDateTime</a>&gt;,&nbsp;</span>","synthetic":false,"types":["time::offset_date_time::OffsetDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.OffsetDateTime.html\" title=\"struct time::OffsetDateTime\">OffsetDateTime</a>&gt; for <a class=\"struct\" href=\"time/struct.OffsetDateTime.html\" title=\"struct time::OffsetDateTime\">OffsetDateTime</a>","synthetic":false,"types":["time::offset_date_time::OffsetDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/std/time/struct.SystemTime.html\" title=\"struct std::time::SystemTime\">SystemTime</a>","synthetic":false,"types":["std::time::SystemTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/std/time/struct.SystemTime.html\" title=\"struct std::time::SystemTime\">SystemTime</a>&gt; for <a class=\"struct\" href=\"time/struct.OffsetDateTime.html\" title=\"struct time::OffsetDateTime\">OffsetDateTime</a>","synthetic":false,"types":["time::offset_date_time::OffsetDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.OffsetDateTime.html\" title=\"struct time::OffsetDateTime\">OffsetDateTime</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/std/time/struct.SystemTime.html\" title=\"struct std::time::SystemTime\">SystemTime</a>","synthetic":false,"types":["std::time::SystemTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.PrimitiveDateTime.html\" title=\"struct time::PrimitiveDateTime\">PrimitiveDateTime</a>","synthetic":false,"types":["time::primitive_date_time::PrimitiveDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.PrimitiveDateTime.html\" title=\"struct time::PrimitiveDateTime\">PrimitiveDateTime</a>","synthetic":false,"types":["time::primitive_date_time::PrimitiveDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.PrimitiveDateTime.html\" title=\"struct time::PrimitiveDateTime\">PrimitiveDateTime</a>&gt; for <a class=\"struct\" href=\"time/struct.PrimitiveDateTime.html\" title=\"struct time::PrimitiveDateTime\">PrimitiveDateTime</a>","synthetic":false,"types":["time::primitive_date_time::PrimitiveDateTime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Time.html\" title=\"struct time::Time\">Time</a>","synthetic":false,"types":["time::time::Time"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Time.html\" title=\"struct time::Time\">Time</a>","synthetic":false,"types":["time::time::Time"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"time/struct.Time.html\" title=\"struct time::Time\">Time</a>&gt; for <a class=\"struct\" href=\"time/struct.Time.html\" title=\"struct time::Time\">Time</a>","synthetic":false,"types":["time::time::Time"]}];
implementors["tokio"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"tokio/io/struct.Ready.html\" title=\"struct tokio::io::Ready\">Ready</a>&gt; for <a class=\"struct\" href=\"tokio/io/struct.Ready.html\" title=\"struct tokio::io::Ready\">Ready</a>","synthetic":false,"types":["tokio::io::driver::ready::Ready"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"tokio/time/struct.Instant.html\" title=\"struct tokio::time::Instant\">Instant</a>&gt; for <a class=\"struct\" href=\"tokio/time/struct.Instant.html\" title=\"struct tokio::time::Instant\">Instant</a>","synthetic":false,"types":["tokio::time::instant::Instant"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.62.1/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.62.1/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"tokio/time/struct.Instant.html\" title=\"struct tokio::time::Instant\">Instant</a>","synthetic":false,"types":["tokio::time::instant::Instant"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()