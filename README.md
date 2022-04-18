# Side Rustle (alpha version)

A simple tool for making async calls to a cli program. The goal is to replicate calls from log files and recreate traffic between environments to help debugging/benchmarking.

```text
result={"success": false}

result={"success": true}

result={"success": false}

Made 3 calls
  successful: 1, failed 2 (33.3%)
```

To do:
* Parse log files
* Logging
* More sophisticated time deltas
* ...
