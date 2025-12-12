(function () {
  var implementors = Object.fromEntries([
    [
      "h3",
      [
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="h3/error/struct.Code.html" title="struct h3::error::Code">Code</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="h3/ext/struct.Protocol.html" title="struct h3::ext::Protocol">Protocol</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" title="trait core::marker::Copy">Copy</a> for <a class="struct" href="h3/quic/struct.StreamId.html" title="struct h3::quic::StreamId">StreamId</a>',
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
//{"start":57,"fragment_lengths":[758]}
