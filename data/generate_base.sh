#!/bin/bash

FLAGS="-s 250"
CHARACTER_TTS="a b c d e f g h i j k l m n o p q r s t u v w x y z space"

for char in ${CHARACTER_TTS};
do
  espeak $FLAGS ${char} -w "${char}.wav"
done
