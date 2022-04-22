# Side Rustle (WIP)

A simple tool for making async calls to a cli program. The goal is to replicate calls from log files and recreate traffic between environments to help debugging/benchmarking.

Sample cli program:

```python
#!/usr/bin/python3
import sys, random, time, random, json

if __name__ == '__main__':
    duration = random.choice(range(2, 5))
    time.sleep(duration)
    print(
        random.choice(
            [json.dumps({"success": True, "slept": duration}), json.dumps({"success": False, "slept": duration})]
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

Parameters:
1) Program to run
2) Parameters to apply
3) Nap time between calls

To do:
* Parse log files
* More sophisticated time deltas
* ...
