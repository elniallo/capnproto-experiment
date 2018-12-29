# Cap'n Proto Experiment
Experimentation with Cap'n proto rpc written in rust as a possible replacement for protocol buffers on the Hycon network. 
<br>
Will be developed concurrently with the [tokio implementation](https://github.com/elniallo/tokio-experiment) and benchmarked accordingly.

## Project Outline
1. Create a basic rpc server and client wth a simple schema
2. Replace simple schema with schema derived from [Hycon Network protocol buffers implementation](https://github.com/Team-Hycon/hycon-core/blob/master/proto/network.proto)
3. Benchmark against Tokio implementation and existing nodejs network