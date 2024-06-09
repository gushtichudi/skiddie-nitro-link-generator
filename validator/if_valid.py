from decoration import Color as c

import os
import sys 

try:
    import requests as r
except ModuleNotFoundError:
    print(f"[!!!] {c.fore}{c.ylw}warning: module `requests` not found. installing...{c.end}")

    install_requests = os.system("pip install requests")

    #with open("log.txt", "w") as log:
    #    log.write(install_requests)

import requests as r

def validate(url):
    print(f"[i] {c.fore}{c.gry}validating {c.fore}{c.blu}{url}...{c.end}")
    try:
        url_status = r.get(url)

        if url_status.status_code == 200:
            print(f"[✓] {c.fore}{c.grn}BRO IT'S FUCKING VALID YOU HAVE NITRO ONSK{c.end}")
            sys.exit(0)
        else:
            print(f"[!!!] {c.fore}{c.red}nah{c.end}")
            sys.exit(1)
    except Exception as e:
        print(f"[!!!] {c.fore}{c.red}nah smth wrong onsk{c.end}")
        sys.exit(1)

def main():
    if len(sys.argv) < 2:
        print("need args")
        sys.exit(1)
    else:
        validate(sys.argv[1])

if __name__ == "__main__":
    main()
