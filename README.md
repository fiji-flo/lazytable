# lazy tables with stuipd wrapping

### Why?

[prettytable](https://github.com/phsym/prettytable-rs) is awesome. But wrapping in a teminal is no fun.

### What can it do?

For now **lazytable** only produces a simple table like this:

Given width of `20`:
```
######################
# da | foobar  | bar #
#    | foobar  |     #
# da | foobar! | bar #
######################
```

Without a width or with [prettytable](https://github.com/phsym/prettytable-rs):
```
######################
# da | foobar foobar #
#| bar               #
# da | foobar! | bar #
######################
```

### TODO

* clean up code
* make it configuarable to some extend
* write proper dock
* publish as crate
