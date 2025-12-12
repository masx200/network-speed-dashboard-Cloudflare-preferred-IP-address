(function () {
  var implementors = Object.fromEntries([
    [
      "h3",
      [
        [
          'impl&lt;C, B&gt; <a class="trait" href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" title="trait core::ops::drop::Drop">Drop</a> for <a class="struct" href="h3/server/struct.Connection.html" title="struct h3::server::Connection">Connection</a>&lt;C, B&gt;<div class="where">where\n    C: <a class="trait" href="h3/quic/trait.Connection.html" title="trait h3::quic::Connection">Connection</a>&lt;B&gt;,\n    B: <a class="trait" href="https://docs.rs/bytes/1.9.0/i686-pc-windows-msvc/bytes/buf/buf_impl/trait.Buf.html" title="trait bytes::buf::buf_impl::Buf">Buf</a>,</div>',
        ],
        [
          'impl&lt;T, B&gt; <a class="trait" href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" title="trait core::ops::drop::Drop">Drop</a> for <a class="struct" href="h3/client/struct.SendRequest.html" title="struct h3::client::SendRequest">SendRequest</a>&lt;T, B&gt;<div class="where">where\n    T: <a class="trait" href="h3/quic/trait.OpenStreams.html" title="trait h3::quic::OpenStreams">OpenStreams</a>&lt;B&gt;,\n    B: <a class="trait" href="https://docs.rs/bytes/1.9.0/i686-pc-windows-msvc/bytes/buf/buf_impl/trait.Buf.html" title="trait bytes::buf::buf_impl::Buf">Buf</a>,</div>',
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
//{"start":57,"fragment_lengths":[1260]}
