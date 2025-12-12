(function () {
  var implementors = Object.fromEntries([
    [
      "h3",
      [
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" title="trait core::convert::From">From</a>&lt;<a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.u64.html">u64</a>&gt; for <a class="struct" href="h3/error/struct.Code.html" title="struct h3::error::Code">Code</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" title="trait core::convert::From">From</a>&lt;<a class="struct" href="h3/error/struct.Code.html" title="struct h3::error::Code">Code</a>&gt; for <a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.u64.html">u64</a>',
        ],
        [
          'impl&lt;B&gt; <a class="trait" href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" title="trait core::convert::From">From</a>&lt;(StreamType, Frame&lt;B&gt;)&gt; for <a class="struct" href="h3/quic/struct.WriteBuf.html" title="struct h3::quic::WriteBuf">WriteBuf</a>&lt;B&gt;<div class="where">where\n    B: <a class="trait" href="https://docs.rs/bytes/1.9.0/x86_64-unknown-linux-gnu/bytes/buf/buf_impl/trait.Buf.html" title="trait bytes::buf::buf_impl::Buf">Buf</a>,</div>',
        ],
      ],
    ],
  ]);
  if (window.register_implementors) {
    window.register_implementors(implementors);
  } else {
    window.pending_implementors = implementors;
  }
})();
//{"start":57,"fragment_lengths":[1224]}
