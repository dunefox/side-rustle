# Side Rustle (WIP)

A simple tool for making async calls to a cli program. The goal is to replicate calls from log files and recreate traffic between environments to help debugging/benchmarking.

Sample cli program:

```python
#!/usr/bin/python3
import sys, random

if __name__ == '__main__':
    print(
        random.choice(
            ["{\"success\": true}", "{\"success\": false}"]
        )
    )
```

Sample output:
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
