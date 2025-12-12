(function () {
  var implementors = Object.fromEntries([
    [
      "h3",
      [
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" title="trait core::hash::Hash">Hash</a> for <a class="enum" href="h3/error/enum.LocalError.html" title="enum h3::error::LocalError">LocalError</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" title="trait core::hash::Hash">Hash</a> for <a class="struct" href="h3/error/struct.Code.html" title="struct h3::error::Code">Code</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" title="trait core::hash::Hash">Hash</a> for <a class="struct" href="h3/quic/struct.StreamId.html" title="struct h3::quic::StreamId">StreamId</a>',
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
//{"start":57,"fragment_lengths":[750]}
