#!/usr/bin/env python3

import requests
import sys

OPENAI_API_KEY = ""

def complete(context: str):
    res = requests.post(
        "https://api.openai.com/v1/completions",
        headers={
            "Authorization": f"Bearer {OPENAI_API_KEY}",
            "Content-Type": "application/json"
        },
        json={
            "model": "text-davinci-003",
            "prompt": context,
            "max_tokens": 100,
            "temperature": 0.2
        }
    )

    return res.json()

def main():
    ctx = " ".join(sys.argv)

    result = complete(ctx)

    print(result["choices"][0]["text"].strip())

if __name__ == '__main__':
    main()
