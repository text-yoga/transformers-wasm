#!/bin/bash
hfdownloader -m TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF:q4_k_m -s ./tests/data -t $TOKEN
hfdownloader -m TinyLlama/TinyLlama-1.1B-Chat-v1.0:tokenizer -s ./tests/data -t $TOKEN
