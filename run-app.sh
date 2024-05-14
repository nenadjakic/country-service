#!/bin/bash

./country-rest-service &
APP1_PID=$!

./country-grpc-service &
APP2_PID=$!

wait $APP1_PID
wait $APP2_PID