# sipfin

## data[done]
* fred (!live)
* yf (!live) 
* sec: 13f(!live)
* nasdaq: rt(live), insider, option-chain

## data[todo]
* todo
--- 
## infra[done]
* google cloud~ 
## infra[todo]
* todo
--- 
## trading[done]
* basic alpaca orders 
## trading[todo]
* portfolio return analysis
* build risk parity package (as generalizable as possible - adam)
--- 
## todo[misc] 
* authorization/security
## todo[prac] 
* map unordered todo to tree, ord, and/or categories 
* alter HasRecs and serialization, avoid `-> V<V<String>>`
* general slash trash & clippy 
* bare minimum tests, docs, benches 

## todo[ord]
1. set up fetch_rt with noria (client server, or instantiate in main?)
	* this will enable changing HasRecs from -> V<V<String>> to ->Vec<Vec<noria::DataType>> (?) 
	
2. how fetch to db works and trash the other fetches
	* decide on how fetching strings should work, could pass it a FnOnce

3. write schemas for ndaq (rt, options, insiders, holdings, financials) 
-- 
queries:
rt: qid: usize, t: DateTime, x: f64, v: u64, (write_time?)
	* all the first models should just use amt of $ mvment for entropy
	* when dS/dt >> 0 => lots of information is being destroyed
	* momentum = mass * dx^2/d^2t right ?? 
	* position is just x_t
	* does sql/noria have eigen/ other linalg fxns
	* look at distribution of obs in p and m (using uncertainty principle) for each company  
--

4. need an actual scheduler; get noria/rt running alongside a client making queries

5. write table(s?) for yf ohlcv's and write query using both rt and yf
 -- cor(take.([rt, yf], n)...)


6. 
* benchmark curl vs fetch unordered 
	* start tracking async benefit 

* figure out wss for apca ? 
	* 

* fix 10-q 

### rt 
-- ndaq, fred rss, 

### daily 


##
```rust
fetch(hm: HashMap<Url, DateTime>) // this might be good
```
it would be nice to have an enum with a fn .to_url() instead of the hashmap  





general first v:
xs = df.o
xs = xs[2:end] - xs[1:end-1]
p(xs_i | xs_(i-1)) # chain lengths can be fib nums, need to write memoized fn 



might be good to look at gifs of how each stocks' singular values evolves in time. 
separable low rank approximations (todo link): (in vid, they serarate X into two matrices)
	* i imagine you can do better by finding more low rank approximation matrices, that when summed still == X
	X = \sum_{i}_{n} Mi 
	where Mi is a low rank data approx and Vi is noise 
	

TODO test neural DE on windows of the singular values 
TODO 

