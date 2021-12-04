#!/bin/bash
#
# Utility which was used on generating the file `.local_secret` .

head -c 100 /dev/random | base64 -w 0
