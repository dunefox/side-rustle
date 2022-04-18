#!/usr/bin/python3
import sys, random

if __name__ == '__main__':
    print(
        random.choice(
            ["{\"success\": true}", "{\"success\": false}"]
        )
    )
