import os
import subprocess

SAMPLE_RATE = 22050
CHS = 1
BITS = 16

def is_wav_file(f):
  s = f.split(".")
  return os.path.isfile(f) and len(s) == 2 and s[1] == "wav"

files = list(filter(is_wav_file, os.listdir()))
sizes = list(map(os.path.getsize, files))

def bytes_to_add(target, current):
  return (target - current) / (SAMPLE_RATE * CHS * BITS / 8)

nb = zip(files, sizes)
target_bytes = max(sizes)
for (name,length) in nb:
  padding = bytes_to_add(target_bytes,length)
  no_ext = name.split(".")[0]
  print(f"{name}: {padding}...", end="")
  subprocess.run(["sox", name, f"{no_ext}_padd.wav", "pad", "0", f"{padding}"])
  subprocess.run(["sox", f"{no_ext}_padd.wav", "-t", "raw", "-r", "{SAMPLE_RATE}", "-b", "{BITS}", "-c", "{CHS}", f"{no_ext}.raw"])
  print("done")

