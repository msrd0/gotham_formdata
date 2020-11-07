(function() {var implementors = {};
implementors["aho_corasick"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["base64"] = [{"text":"impl Error for DecodeError","synthetic":false,"types":[]}];
implementors["bincode"] = [{"text":"impl Error for ErrorKind","synthetic":false,"types":[]}];
implementors["buf_redux"] = [{"text":"impl&lt;W:&nbsp;Any + Send + Debug&gt; Error for IntoInnerError&lt;W&gt;","synthetic":false,"types":[]}];
implementors["chrono"] = [{"text":"impl Error for ParseError","synthetic":false,"types":[]},{"text":"impl Error for RoundingError","synthetic":false,"types":[]}];
implementors["cookie"] = [{"text":"impl Error for ParseError","synthetic":false,"types":[]}];
implementors["futures_channel"] = [{"text":"impl Error for SendError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any&gt; Error for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl Error for Canceled","synthetic":false,"types":[]}];
implementors["futures_executor"] = [{"text":"impl Error for EnterError","synthetic":false,"types":[]}];
implementors["futures_task"] = [{"text":"impl Error for SpawnError","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl Error for Aborted","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any, Item&gt; Error for ReuniteError&lt;T, Item&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any&gt; Error for ReuniteError&lt;T&gt;","synthetic":false,"types":[]}];
implementors["getrandom"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["gotham_formdata"] = [{"text":"impl&lt;Err:&nbsp;Error + 'static&gt; Error for Error&lt;Err&gt;","synthetic":false,"types":[]}];
implementors["h2"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl Error for InvalidHeaderName","synthetic":false,"types":[]},{"text":"impl Error for InvalidHeaderValue","synthetic":false,"types":[]},{"text":"impl Error for ToStrError","synthetic":false,"types":[]},{"text":"impl Error for InvalidMethod","synthetic":false,"types":[]},{"text":"impl Error for InvalidStatusCode","synthetic":false,"types":[]},{"text":"impl Error for InvalidUri","synthetic":false,"types":[]},{"text":"impl Error for InvalidUriParts","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["httparse"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["httpdate"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl Error for InvalidNameError","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl Error for SetLoggerError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelError","synthetic":false,"types":[]}];
implementors["mime"] = [{"text":"impl Error for FromStrError","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Error for LexError","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl Error for BernoulliError","synthetic":false,"types":[]},{"text":"impl Error for WeightedError","synthetic":false,"types":[]},{"text":"impl Error for ReadError","synthetic":false,"types":[]}];
implementors["rand_core"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["rand_jitter"] = [{"text":"impl Error for TimerError","synthetic":false,"types":[]}];
implementors["regex"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for CaseFoldError","synthetic":false,"types":[]},{"text":"impl Error for UnicodeWordError","synthetic":false,"types":[]}];
implementors["serde"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["serde_urlencoded"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["tempfile"] = [{"text":"impl Error for PathPersistError","synthetic":false,"types":[]},{"text":"impl Error for PersistError","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for ConversionRange","synthetic":false,"types":[]},{"text":"impl Error for ComponentRange","synthetic":false,"types":[]},{"text":"impl Error for IndeterminateOffset","synthetic":false,"types":[]},{"text":"impl Error for Format","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl Error for ReuniteError","synthetic":false,"types":[]},{"text":"impl Error for ReuniteError","synthetic":false,"types":[]},{"text":"impl Error for JoinError","synthetic":false,"types":[]},{"text":"impl Error for TryCurrentError","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl Error for ClosedError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for SendTimeoutError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for TryLockError","synthetic":false,"types":[]},{"text":"impl Error for RecvError","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Debug&gt; Error for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Elapsed","synthetic":false,"types":[]}];
implementors["tokio_util"] = [{"text":"impl Error for LengthDelimitedCodecError","synthetic":false,"types":[]},{"text":"impl Error for LinesCodecError","synthetic":false,"types":[]}];
implementors["tracing_core"] = [{"text":"impl Error for SetGlobalDefaultError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelFilterError","synthetic":false,"types":[]}];
implementors["uuid"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()