initSidebarItems({"mod":[["assets","Defines handlers for serving static assets. Defines handlers for static assets, used by `to_file` and `to_dir` routes. Both 'If-None-Match' (etags) and 'If-Modified-Since' are supported to check file modification. Side-by-side compressed files for gzip and brotli are supported if enabled See 'FileOptions' for more details."]],"struct":[["HandlerError","Describes an error which occurred during handler execution, and allows the creation of a HTTP `Response`."]],"trait":[["Handler","A `Handler` is an asynchronous function, taking a `State` value which represents the request and related runtime state, and returns a future which resolves to a response."],["IntoHandlerFuture","Represents a type which can be converted into the future type returned by a `Handler`."],["IntoResponse","Represents a type which can be converted to a response. This trait is used in converting the return type of a function into a response."],["MapHandlerError","This trait allows you to convert a `Result`'s `Err` case into a handler error with the given status code. This is handy if you want to specify the status code but still use the `?` shorthand."],["MapHandlerErrorFuture","This trait allows you to convert a `Result`'s `Err` case into a handler error with the given status code. This is handy if you want to specify the status code but still use the `?` shorthand."],["NewHandler","A type which is used to spawn new `Handler` values. When implementing a custom `Handler` type, this is used to define how instances of the `Handler` are created."]],"type":[["HandlerFuture","A type alias for the trait objects returned by `HandlerService`."],["HandlerResult","A type alias for the results returned by async fns that can be passed to to_async."],["SimpleHandlerResult","A type alias for the results returned by async fns that can be passed to to_async_borrowing."]]});