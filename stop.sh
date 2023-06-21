#!/bin/bash
if [ -e "RUNNING_PID" ]
then
  kill `cat RUNNING_PID`
  rm RUNNING_PID
fi
