#!/usr/bin/env sh

random_sleep_time=$(awk -v min=1 -v max=5 'BEGIN{srand(); print int(min+rand()*(max-min+1))}')
sleep "$random_sleep_time"
# just change to true and save while retry is running
false
