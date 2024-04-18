# Sterile.js

**Sterile** allows you to wrap functions with side effects and turn them into pure functions. 

This is useful for testing, but also for consuming stateful APIs in a purely functional way. 

**Sterile** exposes two functions: `sanitise` and `sterile` (The American English spellings of `sanitize` is` also supported).

## `sanitise()`

`sanitise()` will wrap an existing function and perform a deep copy of all input arguments, includding `this` before calling the wrapped function. The return value will include the original function's return value, as well as `this` and the copies of the input arguments that may have been modified by the wrapped function. For example:

```javascript
function my_function(arr){
    arr.push("hello")
}

let a = [1,2,3];
my_function(a);
console.log(a); // ->  [1, 2, 3, 'hello' ]

let b = [1,2,3];
let clean_function = sanitise(my_function);
let [ret, _this, arg] = clean_function(b);
console.log(b); // ->  [1, 2, 3];

```

`sanitise()` can be used on functions but can also be used on methods, in which case it will defend the containing object from being modified using the `this` keyword.

```javascript
let my_object = {
    foo: 123,
    my_method = function(x){
        this.foo = this.foo * x
    }
}

let my_object.my_method = sanitise(my_object.my_method)
my_object.my_method(100)

console.log(my_object.foo) // 123
```

`sanitise()` will not work on closures, which close over their environments. For example:

```javascript
let a = 1;

let my_function = ()=>{
    a = a+1
    return
}

clean_function = sanitise(my_function)
clean_function()
console.log(a) // -> 2
```

`sanitize()` can get quite expensive in compute and memory, as it will perform a deep copy operation of all the input arguments. 

## `sterile()`

**`sterile()` is a work in progress**

`sterile()` will create a context in which all included code is unable to modify anything outside its context. much like `sanitise()`, it will not prevent closures from accessing variables they close over.

```javascript

let my_data = sterile(()=>{
    //...

    // Code in this scope is unable to affect anything outside of it, including the environment.
    // This includes previously loaded modules and any function that has a side effect
    // standard javascript or nodejs functions and methods that have side effects are not 
    // available within a sterile scope.

    // To pass data out of the sterile environment, return it from this scope:
    return some_data;

    //...
})
```