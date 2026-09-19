# Chapter 1 - What is rust?

## Background

It's exists since 2006 (private project) with first official release at 2012 (public)
Version `1.0` -> First stable release (Backward compatibility)

## What makes rust unique?

Bugs!
Mostly 2 types of bugs

1. Logical - Bad logical flow
2. Undefined Behavior - Code flows that can cause different outputs / actions in different environments - Not defined at language references!

Rust was created in order to solve the 2 type. Meaning that a code that were compiled in rust (regular mode) cannot cause any undefined behavior on any platform / under any situation! Also it's here to prevent data races...
