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
