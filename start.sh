#!/bin/bash
source ./stop.sh
./portrait-booth > application.log 2> error.log &
echo $! > RUNNING_PID
