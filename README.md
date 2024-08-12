

# Rust Pipelines

Stupid repo to play around with rust, mostly exploring type generics, traits, error handling, etc. 
Hoping to explore lifetimes and fully understanding them with deep function and recursive calls.

This repo is just to see if I could implement a simple pipeline strategy to chain together several "tasks" in some arbitrary way,
in this model the tasks should only need the results from the previous task, each message is a simple `HashMap<String, String>`
. Trying to naively think ahead I store the messages as a vector that builds up with each task, I'm hoping this would give a sort
or replayability or some intuition when debugging why tasks might go wrong when they get complex.
