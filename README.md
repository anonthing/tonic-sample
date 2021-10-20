To run:

cargo run --bin tonic-sample --release

To query:
cargo run --bin client --release ip-addr:9999

Using ghz - ghz.sh

ghz <ip-addr>:9999 -c 1 -n 1 --stream-call-count 30 --insecure --proto timeseries.proto --call tonicsample.SampleService.GetResponse -d '{"query": "Tonic"}'
