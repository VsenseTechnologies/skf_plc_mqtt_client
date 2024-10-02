#!/bin/bash

sub_id="vs24skf01"
counter=0
temp=0
pid=0
status=0

while true; do

        mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t vs24skf01/stream_temp -m $temp
        mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t vs24skf01/stream_pid -m $pid

        if [ $counter -eq 10 ]; then
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_temp -m $temp
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_pid -m $pid
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_publish_blower_trip_st -m $status
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_publish_elevator_trip_st -m $status
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_publish_rotor_trip_st -m $status
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_publish_blower_run_st -m $status
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_publish_elevator_run_st -m $status
                mosquitto_pub -h biometricunit.vsensetech.in -p 1883 -t $sub_id/update_publish_rotor_run_st -m $status
                ((status=!status))
                counter=0
        fi
        ((counter++))
        ((temp+=5))
        ((pid+=5))
        ((temp%=80))
        ((pid%=80))
        sleep 1

done
