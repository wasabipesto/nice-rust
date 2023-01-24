# nice-rust

> a client for distributed search of square-cube pandigitals, now with 100% more crab

## Quickstart

Grab the latest release from the [releases page](https://github.com/wasabipesto/nice-rust/releases/latest) and run it with your username as the first and only argument. Optionally, use the username `offline_benchmark` for a prebuilt offline test of 1135123 items. This version should run on most linux builds, please open an issue if you have a problem running it.

## Why does this exist

Square-cube pandigials ("nice" numbers) seem to be distributed pseudo-randomly. It doesn't take very long to check if a number is pandigital in a specific base, but even after we narrow the search range to numbers with the right amount of digits in their square and cube there's a lot of numbers to check. Even worse, it doesn't seem like there are any nice numbers between bases 10 and 140-ish, and it would take an impossibly long time to exhaustively search those at the higher end where the metaphorical ground is more fertile.

Here we have two tricks: data and [dakka](https://tvtropes.org/pmwiki/pmwiki.php/Main/MoreDakka). With enough data on the "niceness" of many numbers, we may be able to find patterns in (or at least take guesses about) which regions are more likely to have nice numbers. Then once we have some tricks, we can start checking semi-randomly in the bases likely to have 100% nice numbers. With enough ~~dakka~~ processing time and luck, anything is possible!

For more background, check out the [original article](https://beautifulthorns.wixsite.com/home/post/is-69-unique) and [my findings](https://wasabipesto.com/nice).

## What this does

This script connects to my server running the [nice-backend](https://github.com/wasabipesto/nice-backend) at `https://nice.wasabipesto.com`. 

When you GET the `/claim` endpoint, the backend returns details of a range to search from the database. Each range is a set of numbers, represented in base 10, alongisde a base to use for representations. The entire possible set of numbers valid in the selected base is divided up into fields of a maximum of 1e9 numbers. This range takes my computer anywhere from 2-7 hours to process, so once you request a range the claim is valid for twelve hours (after which the backend may give this claim to someone else).

When you request a claim, you can optionally include a username in the format `/claim?username=asfaloth`. An example claim response looks like this:

```
{
  "base": 35,
  "claimed_by": "asfaloth",
  "claimed_time": "Wed, 04 Jan 2023 11:43:01 GMT",
  "expiration_time": "Wed, 04 Jan 2023 13:43:01 GMT",
  "search_end": 37069211990,
  "search_id": 120,
  "search_start": 36769211991
}
```

To submit your results, send a POST to `/submit` with the following structure:

```
{
    "search_id": 120, 
    "username": "asfaloth", 
    "client_version": '1.0.0', 
    "unique_count": {
        "1": 0,
        "2": 1,
        ...
    }, 
    "near_misses": {
        "30447607382": 33,
        ...
    }
}
```

- `search_id` should match the ID provided to you in the claim response
- `username` and `client-version` are optional strings for reference only
- `unique_count` is a dict, where
    - the keys are the number of possible unique digits from 1 to the base, and
    - the values are the count of numbers in the range that have that quantity of unique digits
        - the sum of values in this dict should equal the search range
- `near_misses` is a dict, where
    - the keys are numbers who have a niceness >= 0.9 (i.e. the number of unique digits in the representation is greater than/equal to 90% of the max), and
    - the values are the number of unique digits in the representation
        - this dict can be empty if there are no numbers with niceness >= 0

## What you can do

First and foremost, you can run a search node! It doesn't have to be running 24/7, you can shut it down without warning, you are under no obligation to do this for any length of time. Even searching a single range helps!

If you're interested, you can download this souce code and make some tweaks. See if you can reduce the search time, run a node for a while, and see how you stack up. Implement it in another language if you'd like!

## Performance

The main places for optimization seem to be:

- Sorting the digits in the square-cube list, which is required for deduplication in getting the number of unique values. I tried using a HashSet instead, but inserting everything in the set took even longer. A good implementation could cut this down though.
- Taking the input number to the second/third power, which is obviously required and probably cannot be optimized without a massive lookup table. And we never have to do it more than once for any number, so we can't cache results.
- Converting numbers to the desired base, which is again necessary and also abstracted away by `num-bigint`. I don't think I can make this any faster.

![Flamegraph](./flamegraph.svg)