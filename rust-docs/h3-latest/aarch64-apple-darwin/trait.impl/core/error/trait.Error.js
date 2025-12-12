(function () {
  var implementors = Object.fromEntries([
    [
      "h3",
      [
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" title="trait core::error::Error">Error</a> for <a class="enum" href="h3/error/enum.ConnectionError.html" title="enum h3::error::ConnectionError">ConnectionError</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" title="trait core::error::Error">Error</a> for <a class="enum" href="h3/error/enum.StreamError.html" title="enum h3::error::StreamError">StreamError</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" title="trait core::error::Error">Error</a> for <a class="enum" href="h3/quic/enum.ConnectionErrorIncoming.html" title="enum h3::quic::ConnectionErrorIncoming">ConnectionErrorIncoming</a>',
        ],
        [
          'impl <a class="trait" href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" title="trait core::error::Error">Error</a> for <a class="enum" href="h3/quic/enum.StreamErrorIncoming.html" title="enum h3::quic::StreamErrorIncoming">StreamErrorIncoming</a>',
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
//{"start":57,"fragment_lengths":[1116]}
