var searchIndex = {};
searchIndex["asyncplify"] = {"doc":"Functional Reactive Programming (FRP) library (RX like) for rust.","items":[[3,"Clonable","asyncplify","",null,null],[3,"Count","","",null,null],[3,"Debounce","","Only emit an item from a [Stream](./trait.Stream.html) if a particular\nduration has passed without it emitting another item.",null,null],[3,"DedupByKey","","",null,null],[3,"Dedup","","",null,null],[3,"Empty","","A [`Stream`](./trait.Stream.html) that do not emits any element.",null,null],[3,"ParallelEmpty","","A [`ParallelStream`](./trait.ParallelStream.html) that do not emits any element.",null,null],[3,"Filter","","This struct is created by the\n[`filter()`](./trait.Stream.html#method.filter) method on\n[Stream](./trait.Stream.html). See its documentation for more.",null,null],[3,"ParallelFilter","","This struct is created by the\n[`filter()`](./trait.ParallelStream.html#method.filter) method on\n[ParallelStream](./trait.ParallelStream.html). See its documentation for more.",null,null],[3,"Flatmap","","",null,null],[3,"Fold","","",null,null],[3,"Group","","",null,null],[3,"GroupBy","","",null,null],[3,"Inspect","","",null,null],[3,"ParallelInspect","","",null,null],[3,"Interval","","",null,null],[3,"IterStream","","Represent a stream on an iterator.",null,null],[3,"Map","","",null,null],[3,"MaxByKey","","",null,null],[3,"ParallelMaxByKey","","",null,null],[3,"Max","","",null,null],[3,"ParallelMax","","",null,null],[3,"MinByKey","","",null,null],[3,"ParallelMinByKey","","",null,null],[3,"Min","","",null,null],[3,"ParallelObserveOn","","",null,null],[3,"SyncToParallelObserveOn","","",null,null],[3,"Once","","A stream that emits an element exactly once.",null,null],[3,"Scan","","",null,null],[3,"SkipLast","","Ignores the last n items.",null,null],[3,"SkipUntil","","Ignores items until another stream emit one item.",null,null],[3,"Skip","","Ignores the first n items.",null,null],[3,"Sort","","Accumulates all items inside a vec, sort the result and emit all values\nordered.",null,null],[3,"Sum","","Sum all the items and emit only a single item as the total.",null,null],[3,"TakeLast","","Emit the last n items of a stream.",null,null],[3,"TakeUntil","","Emit items until it receive an item from another stream.",null,null],[3,"Take","","Only emit the first n items of a stream.",null,null],[3,"ToVec","","",null,null],[3,"UniqueByKey","","",null,null],[3,"Unique","","",null,null],[3,"Zip","","",null,null],[5,"once","","Creates a stream that emits an element exactly once.",null,{"inputs":[{"name":"t"}],"output":{"name":"once"}}],[11,"new","","",0,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"consume","","",0,{"inputs":[{"name":"clonable"},{"name":"c"}],"output":null}],[11,"clone","","",0,{"inputs":[{"name":"clonable"}],"output":{"name":"self"}}],[11,"drop","","",0,{"inputs":[{"name":"clonable"}],"output":null}],[11,"consume","","",1,{"inputs":[{"name":"count"},{"name":"c"}],"output":null}],[11,"new","","",1,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"new","","",2,{"inputs":[{"name":"s"},{"name":"duration"},{"name":"sc"}],"output":{"name":"self"}}],[11,"consume","","",2,{"inputs":[{"name":"debounce"},{"name":"c"}],"output":null}],[11,"consume","","",3,{"inputs":[{"name":"dedupbykey"},{"name":"c"}],"output":null}],[11,"new","","",3,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",4,{"inputs":[{"name":"dedup"},{"name":"c"}],"output":null}],[11,"new","","",4,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"consume","","",5,{"inputs":[{"name":"empty"},{"name":"c"}],"output":null}],[11,"consume","","",6,{"inputs":[{"name":"parallelempty"},{"name":"c"}],"output":null}],[11,"consume","","",7,{"inputs":[{"name":"filter"},{"name":"c"}],"output":null}],[11,"consume","","",8,{"inputs":[{"name":"parallelfilter"},{"name":"c"}],"output":null}],[11,"new","","",7,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"new","","",8,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"emit","alloc::rc","",9,{"inputs":[{"name":"rc"},{"name":"t"}],"output":{"name":"bool"}}],[11,"new","asyncplify","",10,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",10,{"inputs":[{"name":"flatmap"},{"name":"c"}],"output":null}],[11,"new","","",11,{"inputs":[{"name":"s"},{"name":"o"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",11,{"inputs":[{"name":"fold"},{"name":"c"}],"output":null}],[11,"consume","","",12,{"inputs":[{"name":"group"},{"name":"c"}],"output":null}],[11,"get_key","","",12,{"inputs":[{"name":"group"}],"output":{"name":"k"}}],[11,"clone","","",12,{"inputs":[{"name":"group"}],"output":{"name":"self"}}],[11,"new","","",13,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",13,{"inputs":[{"name":"groupby"},{"name":"c"}],"output":null}],[11,"new","","",14,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"new","","",15,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",14,{"inputs":[{"name":"inspect"},{"name":"c"}],"output":null}],[11,"consume","","",15,{"inputs":[{"name":"parallelinspect"},{"name":"c"}],"output":null}],[11,"new","","Constructs a new `Stream` that emit an empty tuple after each period.",16,{"inputs":[{"name":"duration"}],"output":{"name":"self"}}],[11,"consume","","",16,{"inputs":[{"name":"interval"},{"name":"c"}],"output":null}],[11,"consume","","",17,{"inputs":[{"name":"iterstream"},{"name":"c"}],"output":null}],[11,"new","","",18,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",18,{"inputs":[{"name":"map"},{"name":"c"}],"output":null}],[11,"consume","","",19,{"inputs":[{"name":"maxbykey"},{"name":"c"}],"output":null}],[11,"new","","",19,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"new","","",20,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",20,{"inputs":[{"name":"parallelmaxbykey"},{"name":"c"}],"output":null}],[11,"consume","","",21,{"inputs":[{"name":"max"},{"name":"c"}],"output":null}],[11,"new","","",21,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"consume","","",22,{"inputs":[{"name":"parallelmax"},{"name":"c"}],"output":null}],[11,"new","","",22,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"consume","","",23,{"inputs":[{"name":"minbykey"},{"name":"c"}],"output":null}],[11,"new","","",23,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"new","","",24,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",24,{"inputs":[{"name":"parallelminbykey"},{"name":"c"}],"output":null}],[11,"consume","","",25,{"inputs":[{"name":"min"},{"name":"c"}],"output":null}],[11,"new","","",25,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"new","","",26,{"inputs":[{"name":"s"},{"name":"sc"}],"output":{"name":"self"}}],[11,"new","","",27,{"inputs":[{"name":"s"},{"name":"sc"}],"output":{"name":"self"}}],[11,"consume","","",26,{"inputs":[{"name":"parallelobserveon"},{"name":"c"}],"output":null}],[11,"consume","","",27,{"inputs":[{"name":"synctoparallelobserveon"},{"name":"c"}],"output":null}],[11,"consume","","",28,{"inputs":[{"name":"once"},{"name":"c"}],"output":null}],[11,"new","","",29,{"inputs":[{"name":"s"},{"name":"o"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",29,{"inputs":[{"name":"scan"},{"name":"c"}],"output":null}],[11,"consume","","",30,{"inputs":[{"name":"skiplast"},{"name":"c"}],"output":null}],[11,"new","","",30,{"inputs":[{"name":"s"},{"name":"usize"}],"output":{"name":"self"}}],[11,"consume","","",31,{"inputs":[{"name":"skipuntil"},{"name":"c"}],"output":null}],[11,"new","","",31,{"inputs":[{"name":"s"},{"name":"t"}],"output":{"name":"self"}}],[11,"consume","","",32,{"inputs":[{"name":"skip"},{"name":"c"}],"output":null}],[11,"new","","",32,{"inputs":[{"name":"s"},{"name":"u64"}],"output":{"name":"self"}}],[11,"new","","",33,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"consume","","",33,{"inputs":[{"name":"sort"},{"name":"c"}],"output":null}],[11,"consume","","",34,{"inputs":[{"name":"sum"},{"name":"c"}],"output":null}],[11,"new","","",34,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"consume","","",35,{"inputs":[{"name":"takelast"},{"name":"c"}],"output":null}],[11,"new","","",35,{"inputs":[{"name":"s"},{"name":"usize"}],"output":{"name":"self"}}],[11,"consume","","",36,{"inputs":[{"name":"takeuntil"},{"name":"c"}],"output":null}],[11,"new","","",36,{"inputs":[{"name":"s"},{"name":"t"}],"output":{"name":"self"}}],[11,"consume","","",37,{"inputs":[{"name":"take"},{"name":"c"}],"output":null}],[11,"new","","",37,{"inputs":[{"name":"s"},{"name":"u64"}],"output":{"name":"self"}}],[11,"new","","",38,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",38,{"inputs":[{"name":"tovec"},{"name":"c"}],"output":null}],[11,"consume","","",39,{"inputs":[{"name":"uniquebykey"},{"name":"c"}],"output":null}],[11,"new","","",39,{"inputs":[{"name":"s"},{"name":"f"}],"output":{"name":"self"}}],[11,"consume","","",40,{"inputs":[{"name":"unique"},{"name":"c"}],"output":null}],[11,"new","","",40,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"new","","&#39;Zips up&#39; two streams into a single stream of pairs.",41,{"inputs":[{"name":"l"},{"name":"r"}],"output":{"name":"self"}}],[11,"consume","","",41,{"inputs":[{"name":"zip"},{"name":"c"}],"output":null}],[0,"schedulers","","If you want to introduce multiple threads or delay, you can do so by using schedulers.",null,null],[3,"CurrentThread","asyncplify::schedulers","Schedule a func to be executed on the current thread.",null,null],[3,"EventLoop","","",null,null],[11,"clone","","",42,{"inputs":[{"name":"currentthread"}],"output":{"name":"currentthread"}}],[11,"current","","",42,{"inputs":[],"output":{"name":"currentthread"}}],[11,"running","","",42,{"inputs":[{"name":"currentthread"}],"output":{"name":"bool"}}],[11,"schedule","","",42,{"inputs":[{"name":"currentthread"},{"name":"f"},{"name":"duration"}],"output":null}],[11,"clone","","",43,{"inputs":[{"name":"eventloop"}],"output":{"name":"eventloop"}}],[11,"new","","Creates a new EventLoop",43,{"inputs":[],"output":{"name":"self"}}],[11,"schedule","","",43,{"inputs":[{"name":"eventloop"},{"name":"f"},{"name":"duration"}],"output":null}],[8,"Scheduler","","",null,null],[10,"schedule","","",44,{"inputs":[{"name":"scheduler"},{"name":"f"},{"name":"duration"}],"output":null}],[8,"ParallelScheduler","","",null,null],[10,"schedule","","",45,{"inputs":[{"name":"parallelscheduler"},{"name":"f"},{"name":"duration"}],"output":null}],[8,"Consumer","asyncplify","",null,null],[10,"emit","","",46,{"inputs":[{"name":"consumer"},{"name":"t"}],"output":{"name":"bool"}}],[8,"ParallelConsumer","","",null,null],[10,"emit","","",47,{"inputs":[{"name":"parallelconsumer"},{"name":"t"}],"output":{"name":"bool"}}],[8,"IntoStream","","Extend the Iterator trait with stream conversion operators",null,null],[11,"into_stream","","Convert an iterator to a `Stream`.",48,{"inputs":[{"name":"intostream"}],"output":{"name":"iterstream"}}],[8,"ParallelStream","","",null,null],[16,"Item","","",49,null],[10,"consume","","",49,{"inputs":[{"name":"parallelstream"},{"name":"c"}],"output":null}],[11,"filter","","Creates a stream which uses a closure to determine if an element should\nbe emitted. The closure must return true or false. `filter()` creates a\nstream which calls this closure on each element. If the closure returns\ntrue, then the element is returned. If the closure returns false, it\nwill try again, and call the closure on the next element, seeing if it\npasses the test.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelfilter"}}],[11,"inspect","","Do something with each element of a stream, passing the value on.\nThis is usefull to debug an item.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelinspect"}}],[11,"max","","Returns the maximum element of a stream. Returns the lastest element if\nthe comparison determines two elements to be equally maximum.",49,{"inputs":[{"name":"parallelstream"}],"output":{"name":"parallelmax"}}],[11,"max_by_key","","Returns the element that gives the maximum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally maximum.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelmaxbykey"}}],[11,"min_by_key","","Returns the element that gives the minimum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally minimum.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelminbykey"}}],[11,"observe_on","","",49,{"inputs":[{"name":"parallelstream"},{"name":"sc"}],"output":{"name":"parallelobserveon"}}],[11,"subscribe","","",49,{"inputs":[{"name":"parallelstream"}],"output":null}],[11,"subscribe_action","","",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":null}],[11,"subscribe_func","","",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":null}],[8,"Stream","","",null,null],[16,"Item","","",50,null],[11,"clonable","","Makes the stream clonable for reuse of the output.",50,{"inputs":[{"name":"stream"}],"output":{"name":"clonable"}}],[10,"consume","","",50,{"inputs":[{"name":"stream"},{"name":"c"}],"output":null}],[11,"count","","Count the number of items received.",50,{"inputs":[{"name":"stream"}],"output":{"name":"count"}}],[11,"debounce","","Only emit an item from a [Stream](./trait.Stream.html) if a particular\nduration has passed without it emitting another item.",50,{"inputs":[{"name":"stream"},{"name":"duration"},{"name":"sc"}],"output":{"name":"debounce"}}],[11,"dedup","","Creates a stream that emit only immediate new elements.",50,{"inputs":[{"name":"stream"}],"output":{"name":"dedup"}}],[11,"dedup_by_key","","Creates a stream that emit only immediate new elements.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"dedupbykey"}}],[11,"filter","","Creates a stream which uses a closure to determine if an element should\nbe emitted. The closure must return true or false. `filter()` creates a\nstream which calls this closure on each element. If the closure returns\ntrue, then the element is returned. If the closure returns false, it\nwill try again, and call the closure on the next element, seeing if it\npasses the test.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"filter"}}],[11,"flat_map","","Creates an stream that works like map, but flattens nested structure.\nThe `map()` adapter is very useful, but only when the closure argument\nproduces values. If it produces a stream instead, there&#39;s an extra layer\nof indirection. flat_map() will remove this extra layer on its own.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"flatmap"}}],[11,"fold","","A stream adaptor that applies a function, producing a single, final\nvalue. `fold()` takes two arguments: an initial value, and a closure\nwith two arguments: an &#39;accumulator&#39;, and an element. It returns the\nvalue that the accumulator should have for the next iteration.",50,{"inputs":[{"name":"stream"},{"name":"o"},{"name":"f"}],"output":{"name":"fold"}}],[11,"group_by","","Group incoming values using a `key_selector`.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"groupby"}}],[11,"inspect","","Do something with each element of a stream, passing the value on.\nThis is usefull to debug an item.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"inspect"}}],[11,"into_vec","","Convert the stream into a `Vec`",50,{"inputs":[{"name":"stream"}],"output":{"name":"vec"}}],[11,"last_value","","Returns the last value from stream.",50,{"inputs":[{"name":"stream"}],"output":{"name":"option"}}],[11,"map","","Takes a closure and creates a stream which calls that closure on each\nelement. `map()` transforms one stream into another, by means of its\nargument: something that implements FnMut. It produces a new stream\nwhich calls this closure on each element of the original stream.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"map"}}],[11,"max","","Returns the maximum element of a stream. Returns the lastest element if\nthe comparison determines two elements to be equally maximum.",50,{"inputs":[{"name":"stream"}],"output":{"name":"max"}}],[11,"max_by_key","","Returns the element that gives the maximum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally maximum.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"maxbykey"}}],[11,"min","","Returns the minimum element of a stream. Returns the lastest element if\nthe comparison determines two elements to be equally minimum.",50,{"inputs":[{"name":"stream"}],"output":{"name":"min"}}],[11,"min_by_key","","Returns the element that gives the minimum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally minimum.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"minbykey"}}],[11,"observe_on_parallel","","",50,{"inputs":[{"name":"stream"},{"name":"sc"}],"output":{"name":"synctoparallelobserveon"}}],[11,"scan","","A stream adaptor similar to `fold()` that holds internal state and\nproduces a new stream. `scan()` takes two arguments: an initial value\nwhich seeds the internal state, and a closure with two arguments, the\nfirst being a mutable reference to the internal state and the second an\nstream element. The closure can assign to the internal state to share\nstate between iterations.",50,{"inputs":[{"name":"stream"},{"name":"o"},{"name":"f"}],"output":{"name":"scan"}}],[11,"skip","","Ignore the first X values from the stream",50,{"inputs":[{"name":"stream"},{"name":"u64"}],"output":{"name":"skip"}}],[11,"skip_last","","Ignores the last X values of the stream",50,{"inputs":[{"name":"stream"},{"name":"usize"}],"output":{"name":"skiplast"}}],[11,"skip_until","","Ignores items until the trigger emit a value.",50,{"inputs":[{"name":"stream"},{"name":"u"}],"output":{"name":"skipuntil"}}],[11,"sort","","Sort items from the stream. The stream must terminate somewhere, it\ncannot be an infinite stream here.",50,{"inputs":[{"name":"stream"}],"output":{"name":"sort"}}],[11,"subscribe","","",50,{"inputs":[{"name":"stream"}],"output":null}],[11,"subscribe_action","","",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":null}],[11,"subscribe_func","","",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":null}],[11,"sum","","Sums the elements of a stream.\nTakes each element, adds them together, and returns the result.\nAn empty stream returns the zero value of the type.",50,{"inputs":[{"name":"stream"}],"output":{"name":"sum"}}],[11,"take","","Take only the first X values of the stream and close the stream after",50,{"inputs":[{"name":"stream"},{"name":"u64"}],"output":{"name":"take"}}],[11,"take_last","","Take the only the last X values of the stream and close the stream after",50,{"inputs":[{"name":"stream"},{"name":"usize"}],"output":{"name":"takelast"}}],[11,"take_until","","Take items until the trigger emit a value.",50,{"inputs":[{"name":"stream"},{"name":"u"}],"output":{"name":"takeuntil"}}],[11,"to_vec","","Bundle incoming elements into a `Vec`. A split function can be specified\nto emit a `Vec` when the splitter returns true. The remaing `Vec` is emited\nonly when not empty.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"tovec"}}],[11,"unique","","Creates a stream that emit only new elements. If an element has already\nbeen emitted, it is ignored.",50,{"inputs":[{"name":"stream"}],"output":{"name":"unique"}}],[11,"unique_by_key","","Creates a stream that emit only new elements. If an element has already\nbeen emitted, it is ignored.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"uniquebykey"}}],[11,"zip","","&#39;Zips up&#39; two streams into a single stream of pairs.",50,{"inputs":[{"name":"stream"},{"name":"r"}],"output":{"name":"zip"}}],[11,"into_stream","","Convert an iterator to a `Stream`.",48,{"inputs":[{"name":"intostream"}],"output":{"name":"iterstream"}}],[11,"filter","","Creates a stream which uses a closure to determine if an element should\nbe emitted. The closure must return true or false. `filter()` creates a\nstream which calls this closure on each element. If the closure returns\ntrue, then the element is returned. If the closure returns false, it\nwill try again, and call the closure on the next element, seeing if it\npasses the test.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelfilter"}}],[11,"inspect","","Do something with each element of a stream, passing the value on.\nThis is usefull to debug an item.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelinspect"}}],[11,"max","","Returns the maximum element of a stream. Returns the lastest element if\nthe comparison determines two elements to be equally maximum.",49,{"inputs":[{"name":"parallelstream"}],"output":{"name":"parallelmax"}}],[11,"max_by_key","","Returns the element that gives the maximum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally maximum.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelmaxbykey"}}],[11,"min_by_key","","Returns the element that gives the minimum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally minimum.",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":{"name":"parallelminbykey"}}],[11,"observe_on","","",49,{"inputs":[{"name":"parallelstream"},{"name":"sc"}],"output":{"name":"parallelobserveon"}}],[11,"subscribe","","",49,{"inputs":[{"name":"parallelstream"}],"output":null}],[11,"subscribe_action","","",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":null}],[11,"subscribe_func","","",49,{"inputs":[{"name":"parallelstream"},{"name":"f"}],"output":null}],[11,"clonable","","Makes the stream clonable for reuse of the output.",50,{"inputs":[{"name":"stream"}],"output":{"name":"clonable"}}],[11,"count","","Count the number of items received.",50,{"inputs":[{"name":"stream"}],"output":{"name":"count"}}],[11,"debounce","","Only emit an item from a [Stream](./trait.Stream.html) if a particular\nduration has passed without it emitting another item.",50,{"inputs":[{"name":"stream"},{"name":"duration"},{"name":"sc"}],"output":{"name":"debounce"}}],[11,"dedup","","Creates a stream that emit only immediate new elements.",50,{"inputs":[{"name":"stream"}],"output":{"name":"dedup"}}],[11,"dedup_by_key","","Creates a stream that emit only immediate new elements.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"dedupbykey"}}],[11,"filter","","Creates a stream which uses a closure to determine if an element should\nbe emitted. The closure must return true or false. `filter()` creates a\nstream which calls this closure on each element. If the closure returns\ntrue, then the element is returned. If the closure returns false, it\nwill try again, and call the closure on the next element, seeing if it\npasses the test.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"filter"}}],[11,"flat_map","","Creates an stream that works like map, but flattens nested structure.\nThe `map()` adapter is very useful, but only when the closure argument\nproduces values. If it produces a stream instead, there&#39;s an extra layer\nof indirection. flat_map() will remove this extra layer on its own.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"flatmap"}}],[11,"fold","","A stream adaptor that applies a function, producing a single, final\nvalue. `fold()` takes two arguments: an initial value, and a closure\nwith two arguments: an &#39;accumulator&#39;, and an element. It returns the\nvalue that the accumulator should have for the next iteration.",50,{"inputs":[{"name":"stream"},{"name":"o"},{"name":"f"}],"output":{"name":"fold"}}],[11,"group_by","","Group incoming values using a `key_selector`.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"groupby"}}],[11,"inspect","","Do something with each element of a stream, passing the value on.\nThis is usefull to debug an item.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"inspect"}}],[11,"into_vec","","Convert the stream into a `Vec`",50,{"inputs":[{"name":"stream"}],"output":{"name":"vec"}}],[11,"last_value","","Returns the last value from stream.",50,{"inputs":[{"name":"stream"}],"output":{"name":"option"}}],[11,"map","","Takes a closure and creates a stream which calls that closure on each\nelement. `map()` transforms one stream into another, by means of its\nargument: something that implements FnMut. It produces a new stream\nwhich calls this closure on each element of the original stream.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"map"}}],[11,"max","","Returns the maximum element of a stream. Returns the lastest element if\nthe comparison determines two elements to be equally maximum.",50,{"inputs":[{"name":"stream"}],"output":{"name":"max"}}],[11,"max_by_key","","Returns the element that gives the maximum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally maximum.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"maxbykey"}}],[11,"min","","Returns the minimum element of a stream. Returns the lastest element if\nthe comparison determines two elements to be equally minimum.",50,{"inputs":[{"name":"stream"}],"output":{"name":"min"}}],[11,"min_by_key","","Returns the element that gives the minimum value from the specified\nfunction. Returns the lastest element if the comparison determines two\nelements to be equally minimum.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"minbykey"}}],[11,"observe_on_parallel","","",50,{"inputs":[{"name":"stream"},{"name":"sc"}],"output":{"name":"synctoparallelobserveon"}}],[11,"scan","","A stream adaptor similar to `fold()` that holds internal state and\nproduces a new stream. `scan()` takes two arguments: an initial value\nwhich seeds the internal state, and a closure with two arguments, the\nfirst being a mutable reference to the internal state and the second an\nstream element. The closure can assign to the internal state to share\nstate between iterations.",50,{"inputs":[{"name":"stream"},{"name":"o"},{"name":"f"}],"output":{"name":"scan"}}],[11,"skip","","Ignore the first X values from the stream",50,{"inputs":[{"name":"stream"},{"name":"u64"}],"output":{"name":"skip"}}],[11,"skip_last","","Ignores the last X values of the stream",50,{"inputs":[{"name":"stream"},{"name":"usize"}],"output":{"name":"skiplast"}}],[11,"skip_until","","Ignores items until the trigger emit a value.",50,{"inputs":[{"name":"stream"},{"name":"u"}],"output":{"name":"skipuntil"}}],[11,"sort","","Sort items from the stream. The stream must terminate somewhere, it\ncannot be an infinite stream here.",50,{"inputs":[{"name":"stream"}],"output":{"name":"sort"}}],[11,"subscribe","","",50,{"inputs":[{"name":"stream"}],"output":null}],[11,"subscribe_action","","",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":null}],[11,"subscribe_func","","",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":null}],[11,"sum","","Sums the elements of a stream.\nTakes each element, adds them together, and returns the result.\nAn empty stream returns the zero value of the type.",50,{"inputs":[{"name":"stream"}],"output":{"name":"sum"}}],[11,"take","","Take only the first X values of the stream and close the stream after",50,{"inputs":[{"name":"stream"},{"name":"u64"}],"output":{"name":"take"}}],[11,"take_last","","Take the only the last X values of the stream and close the stream after",50,{"inputs":[{"name":"stream"},{"name":"usize"}],"output":{"name":"takelast"}}],[11,"take_until","","Take items until the trigger emit a value.",50,{"inputs":[{"name":"stream"},{"name":"u"}],"output":{"name":"takeuntil"}}],[11,"to_vec","","Bundle incoming elements into a `Vec`. A split function can be specified\nto emit a `Vec` when the splitter returns true. The remaing `Vec` is emited\nonly when not empty.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"tovec"}}],[11,"unique","","Creates a stream that emit only new elements. If an element has already\nbeen emitted, it is ignored.",50,{"inputs":[{"name":"stream"}],"output":{"name":"unique"}}],[11,"unique_by_key","","Creates a stream that emit only new elements. If an element has already\nbeen emitted, it is ignored.",50,{"inputs":[{"name":"stream"},{"name":"f"}],"output":{"name":"uniquebykey"}}],[11,"zip","","&#39;Zips up&#39; two streams into a single stream of pairs.",50,{"inputs":[{"name":"stream"},{"name":"r"}],"output":{"name":"zip"}}]],"paths":[[3,"Clonable"],[3,"Count"],[3,"Debounce"],[3,"DedupByKey"],[3,"Dedup"],[3,"Empty"],[3,"ParallelEmpty"],[3,"Filter"],[3,"ParallelFilter"],[3,"Rc"],[3,"Flatmap"],[3,"Fold"],[3,"Group"],[3,"GroupBy"],[3,"Inspect"],[3,"ParallelInspect"],[3,"Interval"],[3,"IterStream"],[3,"Map"],[3,"MaxByKey"],[3,"ParallelMaxByKey"],[3,"Max"],[3,"ParallelMax"],[3,"MinByKey"],[3,"ParallelMinByKey"],[3,"Min"],[3,"ParallelObserveOn"],[3,"SyncToParallelObserveOn"],[3,"Once"],[3,"Scan"],[3,"SkipLast"],[3,"SkipUntil"],[3,"Skip"],[3,"Sort"],[3,"Sum"],[3,"TakeLast"],[3,"TakeUntil"],[3,"Take"],[3,"ToVec"],[3,"UniqueByKey"],[3,"Unique"],[3,"Zip"],[3,"CurrentThread"],[3,"EventLoop"],[8,"Scheduler"],[8,"ParallelScheduler"],[8,"Consumer"],[8,"ParallelConsumer"],[8,"IntoStream"],[8,"ParallelStream"],[8,"Stream"]]};
searchIndex["atom"] = {"doc":"","items":[[3,"Atom","atom","An Atom wraps an AtomicPtr, it allows for safe mutation of an atomic\ninto common Rust Types.",null,null],[3,"AtomSetOnce","","This is a restricted version of the Atom. It allows for only\n`set_if_none` to be called.",null,null],[8,"IntoRawPtr","","Convert from into a raw pointer",null,null],[10,"into_raw","","",0,null],[8,"FromRawPtr","","Convert from a raw ptr into a pointer",null,null],[10,"from_raw","","",1,null],[8,"GetNextMut","","This is a utility Trait that fetches the next ptr from\nan object.",null,null],[16,"NextPtr","","",2,null],[10,"get_next","","",2,{"inputs":[{"name":"getnextmut"}],"output":{"name":"nextptr"}}],[11,"fmt","","",3,{"inputs":[{"name":"atom"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"empty","","Create a empty Atom",3,{"inputs":[],"output":{"name":"atom"}}],[11,"new","","Create a new Atomic from Pointer P",3,{"inputs":[{"name":"p"}],"output":{"name":"atom"}}],[11,"swap","","Swap a new value into the Atom, This will try multiple\ntimes until it succeeds. The old value will be returned.",3,{"inputs":[{"name":"atom"},{"name":"p"}],"output":{"name":"option"}}],[11,"take","","Take the value of the Atom replacing it with null pointer\nReturning the contents. If the contents was a `null` pointer the\nresult will be `None`.",3,{"inputs":[{"name":"atom"}],"output":{"name":"option"}}],[11,"set_if_none","","This will do a `CAS` setting the value only if it is NULL\nthis will return `None` if the value was written,\notherwise a `Some(v)` will be returned, where the value was\nthe same value that you passed into this function",3,{"inputs":[{"name":"atom"},{"name":"p"}],"output":{"name":"option"}}],[11,"replace_and_set_next","","Take the current content, write it into P then do a CAS to extent this\nAtom with the previous contents. This can be used to create a LIFO",3,{"inputs":[{"name":"atom"},{"name":"p"}],"output":{"name":"bool"}}],[11,"is_none","","Check to see if an atom is None",3,{"inputs":[{"name":"atom"}],"output":{"name":"bool"}}],[11,"drop","","",3,{"inputs":[{"name":"atom"}],"output":null}],[11,"into_raw","alloc::boxed","",4,null],[11,"from_raw","","",4,null],[11,"into_raw","alloc::arc","",5,null],[11,"from_raw","","",5,null],[11,"fmt","atom","",6,{"inputs":[{"name":"atomsetonce"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"empty","","Create a empty AtomSetOnce",6,{"inputs":[],"output":{"name":"atomsetonce"}}],[11,"new","","Create a new AtomSetOnce from Pointer P",6,{"inputs":[{"name":"p"}],"output":{"name":"atomsetonce"}}],[11,"set_if_none","","This will do a `CAS` setting the value only if it is NULL\nthis will return `OK(())` if the value was written,\notherwise a `Err(P)` will be returned, where the value was\nthe same value that you passed into this function",6,{"inputs":[{"name":"atomsetonce"},{"name":"p"}],"output":{"name":"option"}}],[11,"into_atom","","Convert an AtomSetOnce into an Atom",6,{"inputs":[{"name":"atomsetonce"}],"output":{"name":"atom"}}],[11,"atom","","Allow access to the atom if exclusive access is granted",6,{"inputs":[{"name":"atomsetonce"}],"output":{"name":"atom"}}],[11,"is_none","","Check to see if an atom is None",6,{"inputs":[{"name":"atomsetonce"}],"output":{"name":"bool"}}],[11,"get","","If the Atom is set, get the value",6,{"inputs":[{"name":"atomsetonce"}],"output":{"name":"option"}}],[11,"get_mut","","If the Atom is set, get the value",6,{"inputs":[{"name":"atomsetonce"}],"output":{"name":"option"}}],[11,"dup","","Duplicate the inner pointer if it is set",6,{"inputs":[{"name":"atomsetonce"}],"output":{"name":"option"}}]],"paths":[[8,"IntoRawPtr"],[8,"FromRawPtr"],[8,"GetNextMut"],[3,"Atom"],[3,"Box"],[3,"Arc"],[3,"AtomSetOnce"]]};
initSearch(searchIndex);
