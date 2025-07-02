# Day 2: Finding Nemo

## Challenge
You're given a string of words. You need to find the word "Nemo", and return a string like this: I found Nemo at [the order of the word you find nemo]!.

If you can't find Nemo, return I can't find Nemo :(.

## Requirements
- ! , ? . are always separated from the last word.

- Nemo will always look like Nemo, and not NeMo or other capital variations.

- Nemo's, or anything that says Nemo with something behind it, doesn't count as Finding Nemo.

- If there are multiple Nemo's in the sentence, only return for the first one.

##  My Approach
I wrote a function `find_nemo(sentence: &str)` that splits the sentence into words, removes punctuation, and prints the position of the first "Nemo" found, or a message if none is found.

##  Sample Output
```text
find_nemo("Where is Nemo?") → I found Nemo at 3!
find_nemo("I saw Nemo and then Nemo again.") → I found Nemo at 3!
find_nemo("No fish here!") → I can't find Nemo :(
```
