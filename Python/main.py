#!/usr/bin/python3

from proc import imageProc
import sys

def main():
    print(sys.argv[1])
    imgP = imageProc(sys.argv[1])
    imgP.procecss_image()
    pass

if __name__ == "__main__":
    main()