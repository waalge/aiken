# Moonrat 

> Hedgehog's spineless cousin

## Aims

Property based testing for aiken inspired by hedgehog and elm-test. 

Aims: 

- Default gen and shrinking auto derived for any types
- Support custom gen/shrinking
- Friendly output (progress, sensible feedback such as diffs on large data)
- Reasonably speedy 

Non-aims: 

- e2e testing. 
This is intended for functions rather than testing full txs against validators.
Although it should still be possible, it is not our aim here to make writing and testing txs ergonomic.


## Interface

An aiken file 

```aiken
// my_tests.ak

type T0 {
  f0 : Int, 
  ...
}

fn gen_t0(seed : Int, complexity : Int) -> T0 {
  ...
}

fn shrink_t0(x : T0) -> List<T0> {
  // TODO : what should the signature of this be?! 
  ...
}

type T1 {
  f0 : Int, 
  ...
}

test prop_x ( 
  a0 : T0 via (gen_t0(0), shrink_t0), 
  a1 : T0 via (gen_t0(1), shrink_t0), 
  a2 : T0, 
  a2 : T1, 
) {
  todo!
}
```

Comments on the sample. 
`prop_x` is our test - now supporting arguments. 
There is new syntax `via`.
We have a custom generator and shrinker for `T0` which we may or may not use.
In the absence of a specified gen/shrink pair, the default, autoderived one is used. 

Run 100 times
```
aiken check -m "my_lib/my_test.{prop_x}"
```

Run 1000 cases with a specified seed and shrink limit
```
aiken check --repeat 1000 --seed 123212123 --shrink-limit 5
```

Reporting: 
```sample
Testing ...

my_test 
  prop_x PASS [100/100]  
```

```sample
Testing ...

my_test 
  prop_x FAIL (after 16 tests and 5 shrinks):
  a0 = T0 { f0 : 120201, ... }
  a1 = T0 { ... }
  ... 

  RHS = True 
  LHS = False
  
  seed = 123212123

  Rerun with 
    aiken check -m "my_lib/my_test.{prop_x}" --args " [ T0 { }] ... "
```

## Functionality 

Aiken compiler finds all tests. 
Any tests with args are assumed subject to property based testing.

[Property config](https://hackage.haskell.org/package/hedgehog-1.4/docs/Hedgehog-Internal-Property.html#t:PropertyConfig) is global, rather than local. 

The test is compiled as if it were a parametrized validator. 
Separate gen and shrink functions are also compiled.

To evaluate the test, the generator(s) are run to generate input for the test. 
Then the args are applied, and the code evaluated.
On success this is repeated until `repeat` number of successes. 
On failure, the shrinker is employed to seek a simpler failure case.
