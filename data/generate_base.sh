#!/bin/bash

FLAGS="-s 250"

for letter in {a..z};
do
  espeak $FLAGS ${letter} -w "${letter}.wav"
done
